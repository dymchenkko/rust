use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::ty::is_type_diagnostic_item;
use clippy_utils::{is_in_test_function, is_lint_allowed};
use rustc_hir as hir;
use rustc_lint::LateContext;
use rustc_span::sym;

use super::{EXPECT_USED, UNWRAP_USED};

/// lint use of `unwrap()` for `Option`s and `Result`s
pub(super) fn check(cx: &LateContext<'_>, expr: &hir::Expr<'_>, recv: &hir::Expr<'_>, allow_unwrap_in_tests: bool) {
    let obj_ty = cx.typeck_results().expr_ty(recv).peel_refs();

    let mess = if is_type_diagnostic_item(cx, obj_ty, sym::Option) {
        Some((UNWRAP_USED, "an Option", "None", ""))
    } else if is_type_diagnostic_item(cx, obj_ty, sym::Result) {
        Some((UNWRAP_USED, "a Result", "Err", "an "))
    } else {
        None
    };

    if allow_unwrap_in_tests && is_in_test_function(cx.tcx, expr.hir_id) {
        return;
    }

    if let Some((lint, kind, none_value, none_prefix)) = mess {
        let help = if is_lint_allowed(cx, EXPECT_USED, expr.hir_id) {
            format!(
                "if you don't want to handle the `{none_value}` case gracefully, consider \
                using `expect()` to provide a better panic message"
            )
        } else {
            format!("if this value is {none_prefix}`{none_value}`, it will panic")
        };

        span_lint_and_help(
            cx,
            lint,
            expr.span,
            &format!("used `unwrap()` on `{kind}` value"),
            None,
            &help,
        );
    }
}

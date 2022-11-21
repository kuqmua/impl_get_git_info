#[proc_macro_derive(ImplGetGitInfoFromTufaCommon)]
pub fn derive_impl_get_git_info_from_tufa_common(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(input, proc_macro_helpers::path::Path::TufaCommon)
}

#[proc_macro_derive(ImplGetGitInfoFromCrate)]
pub fn derive_impl_get_git_info_from_crate(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(input, proc_macro_helpers::path::Path::Crate)
}

fn generate(
    input: proc_macro::TokenStream,
    path: proc_macro_helpers::path::Path,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("ImplGetGitInfo syn::parse(input) failed");
    let ident = &ast.ident;
    let git_info_trait_token_stream = format!("{path}::traits::get_git_info::GetGitInfo")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let git_info_trait_usage_token_stream =
        format!("{path}::common::git::git_info::GitInformation")
            .parse::<proc_macro2::TokenStream>()
            .expect("path parse failed");
    let gen = quote::quote! {
        use #git_info_trait_usage_token_stream;
        impl #git_info_trait_token_stream for #ident {
            fn get_git_info(&self) -> &'static GitInformation<'static> {
                &crate::global_variables::compile_time::git_info::GIT_INFO
            }
        }
    };
    gen.into()
}

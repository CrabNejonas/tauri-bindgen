#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod variants {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum E1 {
        A,
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct Empty {}
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct IsClone {
        pub v1: V1,
    }
    pub trait Variants: Sized {
        fn e1_arg(&mut self, x: E1);
        fn e1_result(&mut self) -> E1;
        fn u1_arg(&mut self, x: U1);
        fn u1_result(&mut self) -> U1;
        fn v1_arg(&mut self, x: V1);
        fn v1_result(&mut self) -> V1;
        fn bool_arg(&mut self, x: bool);
        fn bool_result(&mut self) -> bool;
        fn option_arg(
            &mut self,
            a: Option<bool>,
            b: Option<()>,
            c: Option<u32>,
            d: Option<E1>,
            e: Option<f32>,
            f: Option<U1>,
            g: Option<Option<bool>>,
        );
        fn option_result(
            &mut self,
        ) -> (
            Option<bool>,
            Option<()>,
            Option<u32>,
            Option<E1>,
            Option<f32>,
            Option<U1>,
            Option<Option<bool>>,
        );
        fn casts(
            &mut self,
            a: Casts1,
            b: Casts2,
            c: Casts3,
            d: Casts4,
            e: Casts5,
            f: Casts6,
        ) -> (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6);
        fn result_arg(
            &mut self,
            a: Result<(), ()>,
            b: Result<(), E1>,
            c: Result<E1, ()>,
            d: Result<(), ()>,
            e: Result<u32, V1>,
            f: Result<String, Vec<u8>>,
        );
        fn result_result(
            &mut self,
        ) -> (
            Result<(), ()>,
            Result<(), E1>,
            Result<E1, ()>,
            Result<(), ()>,
            Result<u32, V1>,
            Result<String, Vec<u8>>,
        );
        fn return_result_sugar(&mut self) -> Result<i32, MyErrno>;
        fn return_result_sugar2(&mut self) -> Result<(), MyErrno>;
        fn return_result_sugar3(&mut self) -> Result<MyErrno, MyErrno>;
        fn return_result_sugar4(&mut self) -> Result<(i32, u32), MyErrno>;
        fn return_option_sugar(&mut self) -> Option<i32>;
        fn return_option_sugar2(&mut self) -> Option<MyErrno>;
        fn result_simple(&mut self) -> Result<u32, i32>;
        fn is_clone_arg(&mut self, a: IsClone);
        fn is_clone_return(&mut self) -> IsClone;
        fn return_named_option(&mut self) -> Option<u8>;
        fn return_named_result(&mut self) -> Result<u8, MyErrno>;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Variants + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "e1_arg",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: E1,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.e1_arg(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "e1_result",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<E1> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.e1_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "u1_arg",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: U1,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.u1_arg(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "u1_result",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<U1> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.u1_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "v1_arg",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: V1,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.v1_arg(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "v1_result",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<V1> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.v1_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "bool_arg",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: bool,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.bool_arg(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "bool_result",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<bool> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.bool_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "option_arg",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: Option<bool>,
                    b: Option<()>,
                    c: Option<u32>,
                    d: Option<E1>,
                    e: Option<f32>,
                    f: Option<U1>,
                    g: Option<Option<bool>>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.option_arg(a, b, c, d, e, f, g);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "option_result",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<
                    (
                        Option<bool>,
                        Option<()>,
                        Option<u32>,
                        Option<E1>,
                        Option<f32>,
                        Option<U1>,
                        Option<Option<bool>>,
                    ),
                > {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.option_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "casts",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: Casts1,
                    b: Casts2,
                    c: Casts3,
                    d: Casts4,
                    e: Casts5,
                    f: Casts6,
                | -> ::tauri_bindgen_host::anyhow::Result<
                    (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6),
                > {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.casts(a, b, c, d, e, f))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "result_arg",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: Result<(), ()>,
                    b: Result<(), E1>,
                    c: Result<E1, ()>,
                    d: Result<(), ()>,
                    e: Result<u32, V1>,
                    f: Result<String, Vec<u8>>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.result_arg(a, b, c, d, e, f);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "result_result",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<
                    (
                        Result<(), ()>,
                        Result<(), E1>,
                        Result<E1, ()>,
                        Result<(), ()>,
                        Result<u32, V1>,
                        Result<String, Vec<u8>>,
                    ),
                > {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.result_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "return_result_sugar",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Result<i32, MyErrno>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.return_result_sugar())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "return_result_sugar2",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Result<(), MyErrno>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.return_result_sugar2())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "return_result_sugar3",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Result<MyErrno, MyErrno>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.return_result_sugar3())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "return_result_sugar4",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Result<(i32, u32), MyErrno>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.return_result_sugar4())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "return_option_sugar",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Option<i32>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.return_option_sugar())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "return_option_sugar2",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Option<MyErrno>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.return_option_sugar2())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "result_simple",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Result<u32, i32>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.result_simple())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "is_clone_arg",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: IsClone,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.is_clone_arg(a);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "is_clone_return",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<IsClone> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.is_clone_return())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "return_named_option",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Option<u8>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.return_named_option())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "variants",
                "return_named_result",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Result<u8, MyErrno>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.return_named_result())
                },
            )?;
        Ok(())
    }
}

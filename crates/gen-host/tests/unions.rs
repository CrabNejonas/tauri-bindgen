#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod unions {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    ///A union of all of the integral types
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum AllIntegers {
        /**Bool is equivalent to a 1 bit integer
and is treated that way in some languages*/
        Bool(bool),
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        I8(i8),
        I16(i16),
        S32(i32),
        S64(i64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum AllFloats {
        F32(f32),
        F64(f64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum AllText {
        Char(char),
        String(String),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum DuplicatedS32 {
        ///The first s32
        S320(i32),
        ///The second s32
        S321(i32),
        ///The third s32
        S322(i32),
    }
    ///A type containing numeric types that are distinct in most languages
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum DistinguishableNum {
        ///A Floating Point Number
        F64(f64),
        ///A Signed Integer
        S64(i64),
    }
    pub trait Unions: Sized {
        fn add_one_integer(&mut self, num: AllIntegers) -> AllIntegers;
        fn add_one_float(&mut self, num: AllFloats) -> AllFloats;
        fn replace_first_char(&mut self, text: AllText, letter: char) -> AllText;
        fn identify_integer(&mut self, num: AllIntegers) -> u8;
        fn identify_float(&mut self, num: AllFloats) -> u8;
        fn identify_text(&mut self, text: AllText) -> u8;
        fn add_one_duplicated(&mut self, num: DuplicatedS32) -> DuplicatedS32;
        fn identify_duplicated(&mut self, num: DuplicatedS32) -> u8;
        fn add_one_distinguishable_num(
            &mut self,
            num: DistinguishableNum,
        ) -> DistinguishableNum;
        fn identify_distinguishable_num(&mut self, num: DistinguishableNum) -> u8;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Unions + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "unions",
                "add_one_integer",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: AllIntegers,
                | -> ::tauri_bindgen_host::anyhow::Result<AllIntegers> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.add_one_integer(num))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "unions",
                "add_one_float",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: AllFloats,
                | -> ::tauri_bindgen_host::anyhow::Result<AllFloats> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.add_one_float(num))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "unions",
                "replace_first_char",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    text: AllText,
                    letter: char,
                | -> ::tauri_bindgen_host::anyhow::Result<AllText> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.replace_first_char(text, letter))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "unions",
                "identify_integer",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: AllIntegers,
                | -> ::tauri_bindgen_host::anyhow::Result<u8> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.identify_integer(num))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "unions",
                "identify_float",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: AllFloats,
                | -> ::tauri_bindgen_host::anyhow::Result<u8> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.identify_float(num))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "unions",
                "identify_text",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    text: AllText,
                | -> ::tauri_bindgen_host::anyhow::Result<u8> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.identify_text(text))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "unions",
                "add_one_duplicated",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: DuplicatedS32,
                | -> ::tauri_bindgen_host::anyhow::Result<DuplicatedS32> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.add_one_duplicated(num))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "unions",
                "identify_duplicated",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: DuplicatedS32,
                | -> ::tauri_bindgen_host::anyhow::Result<u8> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.identify_duplicated(num))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "unions",
                "add_one_distinguishable_num",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: DistinguishableNum,
                | -> ::tauri_bindgen_host::anyhow::Result<DistinguishableNum> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.add_one_distinguishable_num(num))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "unions",
                "identify_distinguishable_num",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: DistinguishableNum,
                | -> ::tauri_bindgen_host::anyhow::Result<u8> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.identify_distinguishable_num(num))
                },
            )?;
        Ok(())
    }
}

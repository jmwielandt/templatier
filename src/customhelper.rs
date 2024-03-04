use handlebars::{self, JsonTruthy};

pub struct IsDefined;

impl handlebars::HelperDef for IsDefined {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<handlebars::ScopedJson<'rc>, handlebars::RenderError> {
        let func_name = h.clone().name().to_owned();
        if h.params().len() != 1 {
            return Err(handlebars::RenderErrorReason::Other(format!(
                "{func_name} should receive at most 1 parameter."
            ))
            .into());
        }

        if let Some(name) = h.param(0) {
            Ok(handlebars::ScopedJson::Derived(
                handlebars::JsonValue::from(!name.is_value_missing()),
            ))
        } else {
            Err(
                handlebars::RenderErrorReason::ParamNotFoundForName("isdef", "a".to_string())
                    .into(),
            )
        }
    }
}

pub struct IsUndefined;

impl handlebars::HelperDef for IsUndefined {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<handlebars::ScopedJson<'rc>, handlebars::RenderError> {
        let func_name = h.clone().name().to_owned();
        if h.params().len() != 1 {
            return Err(handlebars::RenderErrorReason::Other(format!(
                "{func_name} should receive at most 1 parameter."
            ))
            .into());
        }

        if let Some(name) = h.param(0) {
            Ok(handlebars::ScopedJson::Derived(
                handlebars::JsonValue::from(name.is_value_missing()),
            ))
        } else {
            Err(
                handlebars::RenderErrorReason::ParamNotFoundForName("isundef", "a".to_string())
                    .into(),
            )
        }
    }
}

pub struct IsDefinedPass;

impl handlebars::HelperDef for IsDefinedPass {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<handlebars::ScopedJson<'rc>, handlebars::RenderError> {
        let func_name = h.clone().name().to_owned();
        println!("{}", func_name);
        if h.params().len() != 1 {
            return Err(handlebars::RenderErrorReason::Other(
                "isdef_pass should receive at most 1 parameter.".to_string(),
            )
            .into());
        }

        let include_zero = match h.hash_get("includeZeros") {
            Some(value) => value.value().as_bool().unwrap_or(false),
            None => false,
        };

        if let Some(name) = h.param(0) {
            if name.is_value_missing() {
                return Ok(handlebars::ScopedJson::Derived(
                    handlebars::JsonValue::from(false),
                ));
            }
            Ok(handlebars::ScopedJson::Derived(
                handlebars::JsonValue::from(name.value().is_truthy(include_zero)),
            ))
        } else {
            Err(
                handlebars::RenderErrorReason::ParamNotFoundForName("isdef_pass", "a".to_string())
                    .into(),
            )
        }
    }
}

// #[macro_export]
// macro_rules! my_handlebars_helper {
//     ($struct_name:ident: |$($name:ident: $tpe:tt$(<$($gen:ty),+>)?),*
//      $($(,)?{$($hash_name:ident: $hash_tpe:tt=$dft_val:literal),*})?
//      $($(,)?*$args:ident)?
//      $($(,)?**$kwargs:ident)?|
//      $body:expr ) => {
//         #[allow(non_camel_case_types)]
//         pub struct $struct_name;

//         impl handlebars::HelperDef for $struct_name {
//             #[allow(unused_assignments)]
//             #[allow(unused_variables)]
//             fn call_inner<'reg: 'rc, 'rc>(
//                 &self,
//                 h: &handlebars::Helper<'rc>,
//                 r: &'reg handlebars::Handlebars<'reg>,
//                 _: &'rc handlebars::Context,
//                 _: &mut handlebars::RenderContext<'reg, 'rc>,
//             ) -> std::result::Result<handlebars::ScopedJson<'rc>, handlebars::RenderError> {
//                 let mut param_idx = 0;
//                 dbg!(h.params());
//                 dbg!(h.hash());

//                 let strict_mode: bool = r.strict_mode();

//                 $(
//                     let value = h.param(param_idx);

//                     let $name = h.param(param_idx)
//                         .and_then(|x| {
//                             if x.is_value_missing() {
//                                 None
//                             } else {
//                                 Some(x.value())
//                             }
//                         })
//                         .and_then(|x|
//                             Some(
//                                 handlebars::handlebars_helper!(@as_json_value x, $tpe$(<$($gen),+>)?)
//                                     .ok_or_else(||
//                                         handlebars::RenderErrorReason::ParamTypeMismatchForName(
//                                             stringify!($struct_name),
//                                             stringify!($name).to_string(),
//                                             stringify!($tpe$(<$($gen),+>)?).to_string()
//                                         ).into()
//                                     )
//                             )
//                         );
//                     param_idx += 1;
//                 )*

//                 $(
//                     $(
//                         let $hash_name = h.hash_get(stringify!($hash_name))
//                             .map(|x| x.value())
//                             .map(|x|
//                                     handlebars::handlebars_helper!(@as_json_value x, $hash_tpe)
//                                     .ok_or_else(|| handlebars::RenderErrorReason::HashTypeMismatchForName(
//                                         stringify!($struct_name), stringify!($hash_name).to_string(), stringify!($hash_tpe).to_string()
//                                     ))
//                             )
//                             .unwrap_or_else(|| Ok($dft_val))?;
//                     )*
//                 )?

//                 $(let $args = h.params().iter().map(|x| x.value()).collect::<Vec<&serde_json::Value>>();)?
//                 $(let $kwargs = h.hash().iter().map(|(k, v)| (k.to_owned(), v.value())).collect::<std::collections::BTreeMap<&str, &serde_json::Value>>();)?

//                 let result = $body;
//                 Ok(handlebars::ScopedJson::Derived(handlebars::JsonValue::from(result)))
//             }
//         }
//     };

//     (@as_json_value $x:ident, object) => { $x.as_object() };
//     (@as_json_value $x:ident, array) => { $x.as_array() };
//     (@as_json_value $x:ident, str) => { $x.as_str() };
//     (@as_json_value $x:ident, i64) => { $x.as_i64() };
//     (@as_json_value $x:ident, u64) => { $x.as_u64() };
//     (@as_json_value $x:ident, f64) => { $x.as_f64() };
//     (@as_json_value $x:ident, bool) => { $x.as_bool() };
//     (@as_json_value $x:ident, null) => { $x.as_null() };
//     (@as_json_value $x:ident, Json) => { Some($x) };
//     (@as_json_value $x:ident, $tpe:tt$(<$($gen:ty),+>)?) => { serde_json::from_value::<$tpe$(<$($gen),+>)?>($x.clone()).ok() };
// }

#[derive(Default)]
pub struct D1Arguments(Vec<crate::value::D1Value>);

impl sqlx_core::arguments::Arguments for D1Arguments {
    type Database = crate::D1;

    fn len(&self) -> usize {
        self.0.len()
    }

    fn reserve(&mut self, additional: usize, _size_hint: usize) {
        self.0.reserve(additional);
    }

    fn add<'q, T>(&mut self, value: T) -> Result<(), sqlx_core::error::BoxDynError>
    where
        T: sqlx_core::encode::Encode<'q, Self::Database> + sqlx_core::types::Type<Self::Database>,
    {
        let len_before_encode = self.0.len();
        let _/* IsNull */ = value.encode(&mut self.0)
            .inspect_err(|_| self.0.truncate(len_before_encode))?;
        Ok(())
    }
}

impl sqlx_core::arguments::IntoArguments<crate::D1> for D1Arguments {
    fn into_arguments(self) -> <crate::D1 as sqlx_core::database::Database>::Arguments {
        self
    }
}

impl AsRef<[worker::wasm_bindgen::JsValue]> for D1Arguments {
    fn as_ref(&self) -> &[worker::wasm_bindgen::JsValue] {
        let this: &[crate::value::D1Value] = self.0.as_slice();

        /* SAFETY: `D1Value` is newtype of `JsValue` */
        unsafe { std::mem::transmute(this) }
    }
}

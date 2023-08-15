#![doc = include_str!("../README.md")]

#[cfg(feature = "serde")]
use core::fmt;

#[cfg(feature = "serde")]
use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Deserializer,
    de::{Visitor, Error},
};


#[derive(PartialEq, Debug)]
pub enum DefaultAware<T: Default> {
    Default(T),
    Declared(T),
}

impl<T: Default> DefaultAware<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Default(inner) => inner,
            Self::Declared(inner) => inner,
        }
    }

    pub fn as_ref<'a: 'b, 'b>(&'a self) -> &'b T {
        match self {
            Self::Default(inner) => &inner,
            Self::Declared(inner) => &inner,
        }
    }

    pub fn is_default(&self) -> bool {
        match self {
            Self::Default(_) => true,
            _ => false,
        }
    }

    pub fn is_declared(&self) -> bool {
        !self.is_default()
    }
}

impl<T: Default> Default for DefaultAware<T> {
    fn default() -> DefaultAware<T> {
        DefaultAware::Default(T::default())
    }
}

#[test]
fn default_aware() {
    assert_eq!(DefaultAware::Default(false), DefaultAware::<bool>::default());
    assert_eq!(false, DefaultAware::<bool>::default().unwrap());
    assert_eq!(&false, DefaultAware::<bool>::default().as_ref());

    assert_eq!(Option::<bool>::default(), DefaultAware::<Option<bool>>::default().unwrap());

    assert_eq!(String::default(), DefaultAware::<String>::default().unwrap());

    assert_eq!(u32::default(), DefaultAware::<u32>::default().unwrap());

    assert_eq!(true, DefaultAware::<u32>::default().is_default());
    assert_eq!(false, DefaultAware::<u32>::default().is_declared());

    assert_eq!(false, DefaultAware::Declared(123).is_default());
    assert_eq!(true, DefaultAware::Declared(123).is_declared());
}

#[cfg(feature = "serde")]
struct OptionVisitor<T> {
    marker: PhantomData<T>,
}

#[cfg(feature = "serde")]
impl<'de, T> Visitor<'de> for OptionVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = Option<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("option")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Some)
    }

    fn __private_visit_untagged_option<D>(self, deserializer: D) -> Result<Self::Value, ()>
    where
        D: Deserializer<'de>,
    {
        Ok(T::deserialize(deserializer).ok())
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for DefaultAware<T>
where
    T: Deserialize<'de> + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = OptionVisitor { marker: PhantomData };
        match deserializer.deserialize_option(visitor) {
            Ok(Some(value)) => Ok(DefaultAware::Declared(value)),
            Ok(None) => Ok(DefaultAware::default()),
            Err(err) => Err(err),
        }
    }
}

#[cfg(feature = "serde")]
#[test]
fn default_aware_deserialize() {
    use serde_json::from_str;

    #[derive(Deserialize)]
    struct DefaultAwareTest {
        #[serde(default)]
        foo: DefaultAware<bool>,
    }

    assert_eq!(
        DefaultAware::Default(false),
        from_str::<DefaultAwareTest>("{}").unwrap().foo
    );
    assert_eq!(
        DefaultAware::Declared(false),
        from_str::<DefaultAwareTest>("{ \"foo\": false }").unwrap().foo
    );
    assert_eq!(
        DefaultAware::Declared(true),
        from_str::<DefaultAwareTest>("{ \"foo\": true }").unwrap().foo
    );
}

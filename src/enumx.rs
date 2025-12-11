pub enum Enum1<T1> {
    V1(T1),
}

pub enum Enum2<T1, T2> {
    V1(T1),
    V2(T2),
}

pub enum Enum3<T1, T2, T3> {
    V1(T1),
    V2(T2),
    V3(T3),
}

pub enum Enum4<T1, T2, T3, T4> {
    V1(T1),
    V2(T2),
    V3(T3),
    V4(T4),
}

// Implement From for the first variant for convenience
impl<T1> From<T1> for Enum1<T1> {
    fn from(v: T1) -> Self {
        Enum1::V1(v)
    }
}

impl<T1, T2> From<T1> for Enum2<T1, T2> {
    fn from(v: T1) -> Self {
        Enum2::V1(v)
    }
}

impl<T1, T2, T3> From<T1> for Enum3<T1, T2, T3> {
    fn from(v: T1) -> Self {
        Enum3::V1(v)
    }
}

impl<T1, T2, T3, T4> From<T1> for Enum4<T1, T2, T3, T4> {
    fn from(v: T1) -> Self {
        Enum4::V1(v)
    }
}

#[macro_export]
macro_rules! enumx {
    ($t1:ty) => { $crate::enumx::Enum1<$t1> };
    ($t1:ty | $t2:ty) => { $crate::enumx::Enum2<$t1, $t2> };
    ($t1:ty | $t2:ty | $t3:ty) => { $crate::enumx::Enum3<$t1, $t2, $t3> };
    ($t1:ty | $t2:ty | $t3:ty | $t4:ty) => { $crate::enumx::Enum4<$t1, $t2, $t3, $t4> };
}

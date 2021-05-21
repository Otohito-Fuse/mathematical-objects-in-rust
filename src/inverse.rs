/// 逆元を与える。可逆元の場合はSome(逆元)を返し、そうでない場合はNoneを返す
pub trait Inverse
where
    Self: std::marker::Sized,
{
    fn inverse(self) -> Option<Self>;
}

use arrayvec::ArrayVec;
use rancor::Fallible;

use crate::{
    ser::{Allocator, Writer},
    vec::{ArchivedVec, VecResolver},
    Archive, Archived, Deserialize, Place, Serialize,
};

impl<T, const CAP: usize> Archive for ArrayVec<T, CAP>
where
    T: Archive,
{
    type Archived = ArchivedVec<Archived<T>>;
    type Resolver = VecResolver;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        ArchivedVec::resolve_from_slice(self.as_slice(), resolver, out);
    }
}

impl<T, S, const CAP: usize> Serialize<S> for ArrayVec<T, CAP>
where
    T: Serialize<S>,
    S: Fallible + Allocator + Writer + ?Sized,
{
    fn serialize(
        &self,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(self.as_slice(), serializer)
    }
}

impl<T, D, const CAP: usize> Deserialize<ArrayVec<T, CAP>, D>
    for ArchivedVec<Archived<T>>
where
    T: Archive,
    Archived<T>: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    fn deserialize(
        &self,
        deserializer: &mut D,
    ) -> Result<ArrayVec<T, CAP>, D::Error> {
        let mut result = ArrayVec::new();
        for item in self.as_slice() {
            result.push(item.deserialize(deserializer)?);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use arrayvec::ArrayVec;

    use crate::api::test::roundtrip_with;

    #[test]
    fn roundtrip_array_vec() {
        roundtrip_with(&ArrayVec::<i32, 4>::from([10, 20, 40, 80]), |a, b| {
            assert_eq!(**a, **b)
        });
    }
}

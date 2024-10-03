use crate::{RandomVector, RandomVectorTrait, SequentialVector, SequentialVectorTrait};

// RandomVector += SequentialVector

impl<R: SequentialVectorTrait, V: RandomVectorTrait + std::ops::AddAssign<R>> std::ops::AddAssign<SequentialVector<R>>
    for RandomVector<V>
{
    fn add_assign(&mut self, rhs: SequentialVector<R>) {
        self.object += rhs.object;
    }
}

impl<'a, R: SequentialVectorTrait, V: RandomVectorTrait + std::ops::AddAssign<&'a R>>
    std::ops::AddAssign<&'a SequentialVector<R>> for RandomVector<V>
{
    fn add_assign(&mut self, rhs: &'a SequentialVector<R>) {
        self.object += &rhs.object;
    }
}

// RandomVector += RandomVector

impl<R: RandomVectorTrait, V: RandomVectorTrait + std::ops::AddAssign<R>> std::ops::AddAssign<RandomVector<R>>
    for RandomVector<V>
{
    fn add_assign(&mut self, rhs: RandomVector<R>) {
        self.object += rhs.object;
    }
}

impl<'a, R: RandomVectorTrait, V: RandomVectorTrait + std::ops::AddAssign<&'a R>>
    std::ops::AddAssign<&'a RandomVector<R>> for RandomVector<V>
{
    fn add_assign(&mut self, rhs: &'a RandomVector<R>) {
        self.object += &rhs.object;
    }
}

// RandomVector -= SequentialVector

impl<R: SequentialVectorTrait, V: RandomVectorTrait + std::ops::SubAssign<R>> std::ops::SubAssign<SequentialVector<R>>
    for RandomVector<V>
{
    fn sub_assign(&mut self, rhs: SequentialVector<R>) {
        self.object -= rhs.object;
    }
}

impl<'a, R: SequentialVectorTrait, V: RandomVectorTrait + std::ops::SubAssign<&'a R>>
    std::ops::SubAssign<&'a SequentialVector<R>> for RandomVector<V>
{
    fn sub_assign(&mut self, rhs: &'a SequentialVector<R>) {
        self.object -= &rhs.object;
    }
}

// RandomVector -= RandomVector

impl<R: RandomVectorTrait, V: RandomVectorTrait + std::ops::SubAssign<R>> std::ops::SubAssign<RandomVector<R>>
    for RandomVector<V>
{
    fn sub_assign(&mut self, rhs: RandomVector<R>) {
        self.object -= rhs.object;
    }
}

impl<'a, R: RandomVectorTrait, V: RandomVectorTrait + std::ops::SubAssign<&'a R>>
    std::ops::SubAssign<&'a RandomVector<R>> for RandomVector<V>
{
    fn sub_assign(&mut self, rhs: &'a RandomVector<R>) {
        self.object -= &rhs.object;
    }
}

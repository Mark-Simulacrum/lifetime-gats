use std::marker::PhantomData;
use std::{mem, ops};

/// A type that abstracts over `&'a T` and `T<'a>`, moving the lifetime outside
/// of the type.
///
/// This type is intended for usage by traits which wish to express that their
/// associated type is generic over an input lifetime. That's not possible if
/// you want to return the type directly today, because there's no way to
/// express `type Assoc<'a>;` inside a trait (we need RFC 1598 for that).
///
/// This type is variant over `'a` but invariant over T. It is unsafe to change
/// the lifetime of the type parameter, but just like `&` it is safe to change
/// the lifetime of the reference itself.
// It may seem odd that this has a phantom data with `&'a mut T` but this is
// required because we do not want to be variant over T.
pub struct Reference<'a, T: 'a>(T, PhantomData<&'a mut T>);

/// A type that abstracts over `&'a mut T` and `T<'a>`, moving the lifetime outside
/// of the type.
///
/// See [`Reference`] for more details.
pub struct ReferenceMut<'a, T: 'a>(T, PhantomData<&'a mut T>);

/// A trait implemented by types that can be stored inside [`Reference`] and [`ReferenceMut`].
///
/// `Target` must be the same type as `Self` excluding a lifetime parameter.
///
/// # Example
///
/// ```rust
/// struct SpecialRef<'a>(&'a u32);
///
/// unsafe impl LifetimeCast<'b> for SpecialRef<'a> {
///     type Target = SpecialRef<'b>;
///     unsafe fn cast(self) -> Self::Target {
///         mem::transmute(self)
///     }
///     unsafe fn cast_reference(&self) -> &Self::Target {
///         mem::transmute(self)
///     }
///     unsafe fn cast_reference_mut(&mut self) -> &mut Self::Target {
///         mem::transmute(self)
///     }
/// }
/// ```
pub unsafe trait LifetimeCast<'a>: Sized {
    type Target: 'a;

    /// Casts away the lifetime in `Self` to `'a`.
    ///
    /// Callers must guarantee that this is safe by providing some external
    /// guarantee which restricts the lifetime.
    unsafe fn cast(self) -> Self::Target;

    /// Casts away the lifetime of `Self` to `'a`.
    ///
    /// This is intended for the use case where we need to map `&T<'a>` to `&T<'b>`:
    /// calling `cast` would not allow us to cast the inner type, while this function does.
    unsafe fn cast_reference(&self) -> &Self::Target;

    /// Casts away the lifetime of `Self` to `'a`.
    ///
    /// See `cast_reference` for details.
    unsafe fn cast_reference_mut(&mut self) -> &mut Self::Target;
}

impl<T> Reference<'a, T> {
    /// Create a `Reference` type from some type.
    ///
    /// The returned type (`T::Target`) will be `'static`, which will look odd, as
    /// you'll see `&'static mut T` there in the case of passing a reference.
    ///
    /// However, there's no way to access the data inside this type with
    /// `'static` lifetime, as this type will always return a reference or type
    /// with `'a` lifetime, so this is safe.
    pub fn new(s: T) -> Reference<'a, T::Target>
    where
        T: 'a + LifetimeCast<'static>,
    {
        unsafe { Reference(s.cast(), PhantomData) }
    }
}

impl<'a, T> ReferenceMut<'a, T> {
    /// Create a `ReferenceMut` type from some type.
    ///
    /// See [`Reference`] for more details.
    pub fn new(s: T) -> ReferenceMut<'a, T::Target>
    where
        T: 'a + LifetimeCast<'static>,
    {
        unsafe { ReferenceMut(s.cast(), PhantomData) }
    }
}

impl<T: LifetimeCast<'a> + Copy> ops::Deref for Reference<'a, T> {
    type Target = T::Target;
    fn deref(&self) -> &Self::Target {
        // casting to 'a (the actual lifetime) is always safe
        unsafe {
            self.0.cast_reference()
        }
    }
}

impl<T: LifetimeCast<'a> + Copy> ops::Deref for ReferenceMut<'a, T> {
    type Target = T::Target;
    fn deref(&self) -> &Self::Target {
        // casting to 'a (the actual lifetime) is always safe
        unsafe {
            self.0.cast_reference()
        }
    }
}

impl<T: LifetimeCast<'a> + Copy> ops::DerefMut for ReferenceMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // This overrides the fact that `self` is invariant here (behind &mut).
        // However, because conceptually we're returning the same type we were
        // originally passed this is still a sound operation.
        unsafe {
            self.0.cast_reference_mut()
        }
    }
}

unsafe impl<T: 'static> LifetimeCast<'b> for &'a T {
    type Target = &'b T;
    unsafe fn cast(self) -> &'b T {
        mem::transmute(self)
    }
    unsafe fn cast_reference(&self) -> &Self::Target {
        mem::transmute(self)
    }
    unsafe fn cast_reference_mut(&mut self) -> &mut Self::Target {
        mem::transmute(self)
    }
}

unsafe impl<T: 'static> LifetimeCast<'b> for &'a mut T {
    type Target = &'b mut T;
    unsafe fn cast(self) -> Self::Target {
        mem::transmute(self)
    }
    unsafe fn cast_reference(&self) -> &Self::Target {
        mem::transmute(self)
    }
    unsafe fn cast_reference_mut(&mut self) -> &mut Self::Target {
        mem::transmute(self)
    }
}

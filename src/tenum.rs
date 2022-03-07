// ----- TEnum START -----
pub trait TEnum: PartialEq + Sized {
    fn keys() -> Vec<Self>
    where
        Self: Sized;
}

#[macro_export]
macro_rules! tenum {
    (
	    $([$visibility:vis])? $enum_name:ident {
		    $(
		        $var:ident: $(|&$var_lt:lifetime|)* $var_type:ty = $val:expr
		    ),*
        }
        $(<_> => <$mapped_to_type:ty>)*
    ) => {
	    #[derive(Copy, Clone, PartialEq)]
	    $($visibility)? enum $enum_name {
		    $($var),+
	    }

	    paste ! {
	        impl $enum_name {
		        $(
		            pub const [<$var _ $var_type>]: $(&$var_lt)* $var_type = $val;
		        )+
	        }
		}

	    impl TEnum for $enum_name {
			fn keys() -> Vec<Self> {
				vec![$($enum_name::$var),*]
			}
	    }
    };
}

pub use tenum;
// ----- TEnum END -----

// ----- TEnumMap START -----
pub struct TEnumMap<S: TEnum + Copy, T: Default + Copy> {
    keys: Vec<S>,
    values: Vec<T>,
}

impl<S: TEnum + Copy, T: Default + Copy> TEnumMap<S, T> {
    pub fn empty() -> Self {
        let keys = S::keys();

        let mut values = vec![];
        values.resize_with(keys.len(), || T::default());

        Self { keys, values }
    }

    pub fn new(key_val: &[(S, T)]) -> Self {
        let (keys, values): (Vec<S>, Vec<T>) =
            key_val
                .iter()
                .fold((vec![], vec![]), |(mut acc_k, mut acc_v), (k, v)| {
                    acc_k.push(*k);
                    acc_v.push(*v);

                    (acc_k, acc_v)
                });

        Self { keys, values }
    }

    pub fn get(&self, key: S) -> Option<&T> {
        if let Some(index) = self.index(key) {
            return Some(&self.values[index]);
        }

        None
    }

    pub fn insert(&mut self, key: S, value: T) {
        if let Some(index) = self.index(key) {
            self.values[index] = value;
            return;
        }

        self.keys.push(key);
        self.values.push(value);
    }

    pub fn remove(&mut self, key: S) {
        if let Some(index) = self.index(key) {
            self.keys.remove(index);
            self.values.remove(index);
        }
    }

    #[inline]
    fn index(&self, key: S) -> Option<usize> {
        self.keys.iter().position(|&k| k == key)
    }
}
// ----- TEnumMap END -----

#[macro_export]
macro_rules! alter_by {
    ( $s:expr, $($($f:ident).+ => $v:expr),* ) => {
        {
            let mut new_s = $s.clone();
            $(new_s.$($f).+ = $v;)*
            new_s
        }
    };
}

#[macro_export]
macro_rules! alter_default_by {
    ( $t:ty, $($($f:ident).+ => $v:expr),* ) => {
        {
            let mut new_s = <$t>::default();
            $(new_s.$($f).+ = $v;)*
            new_s
        }
    }
}

#[macro_export]
macro_rules! decode {
    (
        $opcode:expr;
        $self:ident,
        $($code:literal => ($operation:ident, $($addressing:ident)?)),+
    ) => {
        match $opcode {
            $($code => {
                $self.$operation($($self.$addressing())?);
                1 $( - 1 + length::$addressing)?
            })+
            _ => panic!("invalid opcode")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alter_by() {
        #[derive(PartialEq, Debug, Clone, Default)]
        struct TestInner {
            i: i8,
            j: i16,
        };

        #[derive(PartialEq, Debug, Clone, Default)]
        struct TestOuter {
            a: char,
            b: char,
            i: TestInner,
        };

        let test = TestOuter {
            a: 'r',
            b: '0',
            i: TestInner { i: 5, j: 200 },
        };

        let altered = alter_by!(test, a => 'z', i.j => 1600);
        assert_eq!(
            test,
            TestOuter {
                a: 'r',
                b: '0',
                i: TestInner { i: 5, j: 200 }
            }
        );

        assert_eq!(
            altered,
            TestOuter {
                a: 'z',
                b: '0',
                i: TestInner { i: 5, j: 1600 }
            }
        );

        assert_eq!(
            alter_default_by!(TestOuter, a => 'b', i.i => 2),
            TestOuter {
                a: 'b',
                b: '\0',
                i: TestInner { i: 2, j: 0 },
            }
        );
    }
}

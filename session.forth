: . ( x -- ) "Drop and print the top value" %. ;
: . ( Bln -- ) %b. ;
: . ( Int -- ) %i. ;
: . ( Flt -- ) %f. ;
: . ( Str -- ) %s. ;
: . ( Sym -- ) %'. ;
: = ( a b -- Bln ) "Test two values for equality" %.= ;
: = ( Bln Bln -- Bln ) %bb= ;
: = ( Int Int -- Bln ) %ii= ;
: = ( Flt Flt -- Bln ) %ff= ;
: = ( Str Str -- Bln ) %ss= ;
: = ( Sym Sym -- Sym ) %''= ;
: < ( a b -- Bln ) "Test if second value on stack is smaller than the top" "Can't compare" %error ;
: < ( Int Int -- Bln ) %ii< ;
: < ( Flt Flt -- Bln ) %ff< ;
: + ( -- ) "Add two values" "can't add" %error ;
: + ( Int Int -- Int ) %ii+ ;
: + ( Flt Flt -- Flt ) %ff+ ;
: - ( -- ) "Subtract top value from the value below" "can't subtract" %error ;
: - ( Int Int -- Int ) %ii- ;
: - ( Flt Flt -- Flt ) %ff- ;
: * ( -- ) "Multiply two values" "can't multiply" %error ;
: * ( Int Int -- Int ) %ii* ;
: * ( Flt Flt -- Flt ) %ff* ;
: / ( -- ) "Divide second stack value by top" "can't divide" %error ;
: / ( Int Int -- Int ) %ii/ ;
: / ( Flt Flt -- Flt ) %ff/ ;
: and ( Bln Bln -- Bln ) "Logical And" %bb& ;
: or ( Bln Bln -- Bln ) "Logical Or" %bb| ;
: error ( Str -- ) "Report an error" %error ;
: >> ( x -- ) "Move value to secondary stack" %>> ;
: << ( -- x ) "Move value from secondary stack" %<< ;
: drop ( x -- ) "Remove top value" %drop ;
: dup ( x -- x x ) "Duplicate top value" %dup ;
: swap ( a b -- b a ) "Swap the two top-most values on the stack" %swap ;
: rot ( a b c -- b c a ) "Rotate the third value on the stack to the top" %rot ;
: dup2 ( a b -- a b a b ) swap dup rot dup rot swap ;
: fib ( Int -- Int ) "Compute the n-th fibonacci number" %dup 2 %ii< if [ %drop 1 ] [ %dup 1 %ii- fib %swap 2 %ii- fib %ii+ ] ;
:t Complex "complex number"  Flt Flt ;
: tuck-real ( Complex -- Complex ) #1 >> ;
: tuck-imag ( Complex -- Complex ) #2 >> ;
: + ( Complex Complex -- Complex ) tuck-imag swap tuck-imag tuck-real drop tuck-real drop << << + << << + Complex ;
: - ( Complex Complex -- Complex ) tuck-imag swap tuck-imag tuck-real drop tuck-real drop << << - << << - Complex ;
: * ( Complex Complex -- Complex ) tuck-imag swap tuck-real tuck-imag swap tuck-real tuck-imag swap tuck-imag tuck-real drop tuck-real drop << << * << << * - << << * << << * + Complex ;
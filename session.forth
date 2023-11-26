: PI ( -- Flt ) 3.141592653589793 ;
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
: over ( a b -- a b a ) "Copy the second value on the stack" %over ;
: dup2 ( a b -- a b a b ) swap dup rot dup rot swap ;
: sqr ( x -- x ) "Square a number" dup * ;
: sqrt ( Flt -- Flt ) "Square root" %fsqrt ;
: log ( Flt -- Flt ) "Square root" %flog ;
: sin ( x -- x ) "Sine" %fsin ;
: cos ( x -- x ) "Cosine" %fcos ;
: fib ( Int -- Int ) "Compute the n-th fibonacci number" %dup 2 %ii< if [ %drop 1 ] [ %dup 1 %ii- fib %swap 2 %ii- fib %ii+ ] ;
:t Complex "complex number"  Flt Flt ;
: f->c ( Flt -- Complex ) 0.0 Complex ;
: i->c ( Int -- Complex ) %i->f f->c ;
: polar->c ( Flt Flt -- Complex ) dup2 sin * >> cos * << Complex ;
: . ( Complex -- ) #2 swap #1 swap drop "%f+%fi" %fmt %println ;
: tuck-real ( Complex -- Complex ) #1 >> ;
: tuck-imag ( Complex -- Complex ) #2 >> ;
: + ( Int Complex -- Complex ) swap i->c + ;
: + ( Flt Complex -- Complex ) swap f->c + ;
: - ( Int Complex -- Complex ) swap i->c swap - ;
: - ( Flt Complex -- Complex ) swap f->c swap - ;
: * ( Int Complex -- Complex ) swap i->c * ;
: * ( Flt Complex -- Complex ) swap f->c * ;
: + ( Complex Int -- Complex ) i->c + ;
: + ( Complex Flt -- Complex ) f->c + ;
: - ( Complex Int -- Complex ) i->c - ;
: - ( Complex Flt -- Complex ) f->c - ;
: * ( Complex Int -- Complex ) i->c * ;
: * ( Complex Flt -- Complex ) f->c * ;
: / ( Complex Int -- Complex ) %i->f / ;
: / ( Complex Flt -- Complex ) 1.0 swap / * ;
: + ( Complex Complex -- Complex ) tuck-imag swap tuck-imag tuck-real drop tuck-real drop << << + << << + Complex ;
: - ( Complex Complex -- Complex ) tuck-imag swap tuck-imag tuck-real drop tuck-real drop << << - << << - Complex ;
: * ( Complex Complex -- Complex ) tuck-imag swap tuck-real tuck-imag swap tuck-real tuck-imag swap tuck-imag tuck-real drop tuck-real drop << << * << << * - << << * << << * + Complex ;
: abs ( Complex -- Flt ) "Complex magnitude" #1 sqr swap #2 sqr swap drop + sqrt ;

: apply ( Ops -- ) "Apply a block of code" %apply ;
: repeat ( Ops Int -- ) dup 0 = if [ drop drop ] [ >> dup >> apply << << 1 - repeat ] ;

: c ( -- Complex ) 0 %@ ;
: x ( -- Complex ) 1 %@ ;
: dx ( -- Complex ) 2 %@ ;
: mandel-prep ( Complex Complex Complex -- ) "prepare variables so the accessors can fin them" >> >> >> ;
: mandel-clean ( -- ) "clean up secondary stack" << drop << drop << drop ;
: mandel-update-x ( -- ) x sqr c + ;
: mandel-update-dx ( -- ) 2 x dx * * 1 + ;
: mandel-update ( Complex Complex Complex -- Complex Complex Complex ) "One iteration step" mandel-prep c mandel-update-x mandel-update-dx mandel-clean ;
: mandel-dist ( Complex Complex Complex -- Flt ) "Estimate distance" mandel-prep c x abs dup log 2.0 * * dx abs / mandel-clean ;
: mandel-step ( Complex Complex Complex -- Complex Complex Complex ) mandel-update dup2 mandel-dist 4.0 / mandel-grid-scale / . ;
: mandel-grid-scale ( -- Float ) 2.0 13.0 / ;
: mandel-grid->polar ( Int Int -- Complex ) "convert r and steps around the radius to complex number" %i->f swap %i->f swap over 6.0 2.0 / * / PI * swap mandel-grid-scale * swap ;
: mandel-init ( Int Int -- Complex Complex Complex ) mandel-grid->polar polar->c dup 1.0 0.0 Complex ;

: mandel ( Int Int -- ) mandel-init [ mandel-step ] 20 repeat drop drop drop ;
: info ( -- ) " < 1 ... black, < 4 ... pink, >= 4 white " ;

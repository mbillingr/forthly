# Forthly

This is a variant of the Forth programming language. It has the following distinguishing features:
- First-class code blocks generalize control flow
- User-defined data types
- Function overloading
- Persistent interaction - new definitions are appended to a session file and loaded at the next start

## Get Started

### Basics Usage
Perform simple calculations in the REPL with reverse Polish notation.
```
1 2 3 * + .
```
Computes 2 * 3 + 1 and displays the result, 7. In particular, the values 1, 2, and 3 are pushed to the stack in order. 
Then, `*` pops the two top-most values from the stack and pushes the result of the multiplication back on the stack. 
Next, `+` pops 6 and 1 from the stack and pushes 7. Finally, `.` pops a value from the stack and displays it. 

### Language Elements
Numbers like `1`, `3` or `486423745` are integer literals. Other literals are floating point numbers like `3.1415` and 
strings `"Hellow world!"`. Literals represent values, which are pushed to the stack when encountered by the interpreter.

Most other elements are function names, or in Forth lingo, *words*. Words can contain almost any combination of 
characters other than whitespace and string delimiters, and they cannot form valid numbers.
When the interpreter encounters a word, it looks for the function definition and executes its code before continuing 
with the next instruction. Functions typically pop their arguments from the stack and push results back.

There are a few special forms, like `:`, `if`, and some others. They behave differently than normal words and will be 
described individually below.

The builtin `:words` displays all currently defined words and `:stacks` shows the current content of (both) stacks.

### Control Flow
The `if` command is special. Its syntax is
```
if then-block else-block
```
So, in contrast to normal syntax, which follows a postfix notation, where the operands precede the operator
(i.e. `1 2 +`), the two blocks come after the `if`.

When the interpreter encounters an if, it pops a boolean from the stack. If it is `true`, the `then-block` is executed, 
otherwise the `else` block. Blocks are denoted by wrapping instructions in `[` and `]`.

Example:
```
0 = if [ "it's zero!" . ] [ "nonzero" . ]
```

### Defining New Words
The `:` word starts a new definition. It's full syntax is
```
: name ( stack-effect ) "optional docstring" instruction instruction ... ;
```
Where `name` is the name of the word being defined. The stack effect describes what the function expects to find on the
stack and what it pushes back. The rest of the definition consists of a sequence of instructions, ended by `;`. If the
first instruction is a string literal, it is not part of the function body but serves as documentation.

Stack effects are of the form `( elem ... -- elem ... )`: two lists of elements, separated by `--`. The first list 
represents the top of the stack before the function executes, and the second list represents the stack after the 
function executed. Each stack element `elem` can be a type or an identifier. Types start with uppercase characters and 
identifiers with lowercase characters. The stack effect serves mostly documentation purposes. It is neither checked nor 
enforced. However, the stack effect declaration participates in dispatching overloaded functions.

Example:
```
: sqr ( Int -- Int ) "square an integer number" dup * ;
```

### Function Overloading
If a function is defined again, it does not in general replace the previous definition.
Instead, the types declared in their stack effect are compared against the actual types of values on the stack. The 
interpreter selects the last definition whose types match.

For example,

```
: sqr ( x -- x ) "generic square" dup * ;
: sqr ( Int -- Int ) "square an integer" dup %i* ;
: sqr ( Flt -- Flt ) "square a float" dup %f* ;
```
with these definitions `2 sqr` and `2.1 sqr` invoke their respective specialized implementations, while `"hi" sqr` 
invokes the generic implementation where no type was specified (which likely fails, unless an implementation of `*` for
strings was defined too).

### Defining new types
The syntax for defining types (named tuples) is
```
:t Name "docstring" Type Type ... ;
```
Where `Name` is the name of the type, starting with an uppercase character.
Again, the docstring is optional, and finally, the sequence of type declares the type of each field in the tuple.

To construct a user defined type, invoke `Name`. It pops as many values from the stack as fields were declared. Then it 
pushes one value of type `Name` back on the stack.

Use `#1` to access the first field, `#2` for the second field, and so on. These operators expect a tuple value on the 
stack and push the field value on top, leaving the tuple in place.

Example:
```
:t Vec2 "Vector with two integer elements" Int Int ;
: + ( Vec2 Vec2 -- Vec2 ) "overload + for vectors" #1 >> #2 >> drop #1 >> #2 >> drop << << << << rot + rot rot + Vec2 ;
```

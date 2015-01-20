/// `mod` is similar to `package` in Java, it declares a module namespace.
///
/// `pub` is short for "public" and means that this module may have contents
/// that are accessible by code in other modules.
///
/// This particular module represents what I would consider a reasonably
/// balanced approach to the posed challenge.
pub mod reasonable_implementation {

    /// The `use` keyword is for importing functionality from other modules.
    /// Note the double-colon `::` style of separating out namespace trees.
    use std::vec::Vec;
    /// Here we import multiple types from a module
    use std::collections::hash_map::{HashMap, Entry};
    use std::iter::AdditiveIterator;

    /// A trait resembles a Java `interface` in so far as it is composed
    /// of a series of function signatures that may be implemented
    /// elsewhere by other types.
    pub trait Market {

        /// Calculates the price of a sequence of items encoded in a string.
        /// 
        /// # Arguments
        /// 
        /// * `self` - a reference to the implementing type's instance
        /// * `items` - a product list, where each item purchased is
        /// assumed to be represented by a single character.
        ///
        /// # Returns
        /// A value in whole dollars.
        ///
        /// The size of the integer chosen matches the default integer size
        /// in the Java prompt, and correspondingly limits the absolute quantity
        /// calculable. Luckily, a limit of around 2 billion is a
        /// reasonable match for the domain. A still better match
        /// would have been to use cents, but that would violate the
        /// challenge prompt's constraints.
        fn checkout(&self, items:String) -> i32;
    }

    /// A `struct` is the datatype most similar to a Java class,
    /// as it is a data structure with named member fields
    /// and associated  associated functions.
    ///
    /// The `pub` keyword indicates that this struct is visible to
    /// code in other modules. Structs, traits, functions, and members
    /// are private by default, and only made visible by the addition of
    /// the `pub` keyword.
    ///
    /// The Supermarket struct is the sole visible structure in this module.
    ///
    /// The `'s` in angle brackets after the struct's name is a lifetime name.
    /// Lifetimes are used to track how long given objects are valid and in-use.
    /// Here, we define a new lifetime, named "s" for terseness.  This lifetime
    /// will be used to help clarify to the compiler that other instances are
    /// expected to stay alive during the same parts of the code as a
    /// Supermarket instance.
    pub struct Supermarket<'s> {

        /// `price_rules` is a Vector of `Box<PricingRule>` instances. `Vector`
        /// is a simple resizable linear collection, akin to a Java `List`.
        ///
        /// Like a Java list, Vec is generic, meaning it can be used with
        /// a user-specified particular type. Its element type is 
        /// specified using angle-bracket notation, e.g. Vec<ElementType>
        ///
        /// The `+ 's` portion below makes sure the PricingRules held in this
        /// Vec must be alive while this Supermarket is alive.
        ///
        /// This `price_rules` member does not have a `pub` prefix, and thus
        /// is not visible to or editable by  code outside of this module.
        ///
        /// We gained access to Vec thanks to the `use std::vec::Vec;` import
        /// statement up at the top of this module.
        price_rules: Vec<Box<PricingRule + 's>>
    }

    /// Implementation of general-purpose functions for the Supermarket type.
    ///
    /// The `impl` keyword means the beginning of an implementation block,
    /// wherein functions are defined that are relevant to the data type
    /// in question.
    ///
    /// This is different from Java, where methods for a class must be
    /// defined in the main body of the class.
    ///
    /// Note also that we're chaining through a lifetime definition, `'s`.
    impl<'s> Supermarket<'s> {

        /// A public constructor for the `Supermarket` struct.
        /// This allows external code to create and use a `Supermarket`
        /// even though it lacks access to its private `price_rules` field.
        pub fn new() -> Supermarket<'s> {
            
            // Here we instantiate the members-to-be of the pricing rules.
            // The use of a `Box::new` call wrapping the individual structs
            // clarifies that those structures should be allocated on the
            // heap, and a reference provided to those structures.
            let a = Box::new(FlatPrice { 
                product: 'A',
                cost: 20
            }) as Box<PricingRule>;

            // `as MyType` is a cast in Rust, equivalent to `(MyType) obj`
            // in Java-land. The reason for the casting here is to
            // assist the `vec!` call below in appropriately picking the
            // right type of collection to make. If we weren't interested
            // in demonstrating casting for educational reasons, we might
            // use a different formulation that was less explicit.
            let b = Box::new(BundlePrice {
                product: 'B',
                lone_cost: 50,
                bundle_size: 5,
                bundle_cost: 150
            }) as Box<PricingRule>;
            
            // Another important difference between Java and Rust is
            // that these `let value_name` variables are immutable by default.
            // This is a lot like having `final` variables everywhere.
            let c = Box::new(FlatPrice {
                product: 'C',
                cost: 30
            }) as Box<PricingRule>;

            // The last expression in a function is returned automatically
            // without requiring a `return` keyword.
            Supermarket {
                // `vec!` is a macro which generates a Vec of a type
                // matching the input list.
                price_rules: vec!(a, b, c)
            }
        }

        fn count_characters(items:String) -> HashMap<char, i32> {
            // `let mut` means that this variable is mutable.
            let mut count = HashMap::new();

            // A `for` loop in Rust makes use of iterators. In this case,
            // `items.chars()` is producing an iterator, which produces
            // references to the elements of the thing we're iterating over,
            // namely the characters from the `items` String.
            for c in items.chars() {

                // `match` is like a `switch` statement on steroids. It checks
                // at compile time that we've handled all possible cases.
                //
                // Importantly, `match` can be used with complex data types
                // like `enum` and even `struct`s. Like Java, it can also do
                // primitive types.
                //
                // A HashMap's `entry` method returns an `enum` of type `Entry`
                // so here we get confirmation that both of the possible `enum` 
                // options have cases.  
                match count.entry(c) {
                    // cases are specified with the value that should be matched
                    // followed by an `=>` arrow, then an expression.
                    // This case statement matches when `count.entry(c)` returns
                    // the `Vacant` enum value. The `Vacant` option includes a
                    // wrapped reference to the slot in the HashMap in question.
                    Entry::Vacant(slot) => {
                        // We only get here when the slot was empty, meaning no
                        // characters matching this one have been found yet,
                        // so we can insert a count of 1
                        slot.insert(1);
                    },
                    // cases are separated by commas. The practice of breaking
                    // out the matching value's type and its component members
                    // (here, the `slot`) is called destructuring.
                    Entry::Occupied(mut slot) => {
                        // This slot is occupied, meaning some previous matches
                        // have already been found for this character. Increment
                        // the count by one.
                        *slot.get_mut() += 1;
                    }
                }
            }

            // Return the HashMap. Note the lack of a semicolon. semicolons are
            // used for to split rust expressions into distinct statements.
            // Statements produce the unit type `()`, which is a lot like `void`
            // in Java.
            //
            // Because we actually want to return a value here, we want an
            // expression, not a statement on the last line of the function.
            count
        }
    }

    /// An implementation of the `Market` trait for the `Supermarket` struct
    ///
    /// Unlike the preceding `impl` block where any function could be added,
    /// we are constrained to defining *just* the functions relevant to the
    /// Market trait.
    impl<'s> Market for Supermarket<'s> { 
        /// The `&` preceding the `self` parameter clarifies that we're being
        /// provided a reference to the Supermarket struct instance, not
        /// the direct value.
        ///
        /// References, and more broadly, pointers, are deep topics
        /// that deserve more of an explanation than you'll presently get here.
        /// Check out http://doc.rust-lang.org/book/pointers.html for more.
        ///
        /// You can think of functions with the `&self` parameter as being like
        /// methods, whereas functions without it are more like static functions
        /// in Java.
        fn checkout(&self, items:String) -> i32 {
            // Note that we can make use of private functions from the
            // Supermarket `impl` block because we are in the same module.
            //
            // The count_characters function was defined without a `&self` param
            // so we call it with the following TypeName::function_name(args)
            // syntax.
            let counts = Supermarket::count_characters(items);

            // Here we see a hint at the functional-style terseness possible
            // in Rust. The next expression iterates through the price rules,
            // runs a fresh function (defined inline) on each of the rules,
            // and sums up the individual results.
            //
            // The inline (a.k.a "anonymous") function definition syntax used
            // is simply `|parameter_name| expression`
            //
            // If multiple lines were needed, it could have also been written
            // `|parameter_name| { ... multiple lines ... }`
            self.price_rules.iter().map(|p| p.price(&counts) ).sum()
        }
    }
    
    /// Provides a means of attaching a price to some subset of the items.
    ///
    /// From a design perspective, a more complicated and robust solution
    /// might include additional functions or return values to specify
    /// which of the input items were actually accounted-for by this rule
    /// in order to discover un-priced items. For the sake of simplicity,
    /// these have been omitted.
    trait PricingRule {
        ///
        /// # Arguments
        /// 
        /// * `character_counts` - the number of instances of each character
        /// found in the `items` input String to the `Market.checkout` function.
        /// Note that because this input is a simple map of counts, any ordering
        /// of characters found in the original string has been lost, so
        /// sequence-order-dependent pricing rules are not expressable with
        /// this interface formulation.
        ///
        /// # Returns
        /// The price of the items that this rule is accounting for.
        /// This number may be negative, possibly useful for indicating some
        /// discount, coupon, or combo deal.
        fn price(&self, character_counts:&HashMap<char, i32>) -> i32;
    }
    
    /// Represents a simple flat price. For every item matching the product,
    /// the cost is added to the price.
    struct FlatPrice {
        product: char,
        cost: i32
    }

    impl PricingRule for FlatPrice {
        fn price(&self, character_counts:&HashMap<char, i32>) -> i32 {
            // The `get` method of a HashMap returns an Option<T>, which is
            // an enum with two possibilities, either None or Some(x),
            // where x is a reference to a value of type T.
            //
            // In this case, T is the count for that character.  
            match character_counts.get(&self.product) {
                Some(&count) => count * self.cost,
                // No key was found that matched the product character code,
                // so there's no cost.
                None => 0
            }
            // `match` produces the value of the selected case's expression.
            // We could store that value in a `let` variable, or, if the match
            // is the last thing in the function, it gets returned.
        }
    }

    /// Represents a price for a product where you can buy it in bundles of 
    /// a set size for one cost, but you have a number of items less than a
    /// bundle, there is a different cost.  Allows for unlimited bundles.
    ///
    /// Equivalent to "X cost apiece, or Y cost when you buy N of them"
    struct BundlePrice {
        product: char,
        lone_cost: i32,
        bundle_size: i32,
        bundle_cost: i32
    }

    impl PricingRule for BundlePrice {
        fn price(&self, character_counts:&HashMap<char, i32>) -> i32 {
            match character_counts.get(&self.product) {
                // Here we match on an exact value, 0, rather than capturing
                // the integer into a variable name (as is done in the 2nd case)
                Some(&0) => 0,
                Some(&non_zero_count) => {
                    let bundles = non_zero_count / self.bundle_size;
                    let leftovers = non_zero_count % self.bundle_size;
                    bundles * self.bundle_cost + leftovers * self.lone_cost
                },
                None => 0
            }
        }
    }
}

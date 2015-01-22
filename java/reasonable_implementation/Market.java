package reasonable_implementation;

/**
 * The primary purpose of splitting out Market into a separate
 * interface/trait is to easily enable shared testing between
 * multiple implementations. Currently, only one implementation
 * exists, so this may be considered premature abstraction.
 */
public interface Market {
    public int checkout(String items);
}
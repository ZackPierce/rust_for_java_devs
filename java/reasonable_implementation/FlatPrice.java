package reasonable_implementation;

import java.util.Map;

class FlatPrice implements PricingRule {
    final private char product;
    final private int cost;

    FlatPrice(char product, int cost) {
        this.product = product;
        this.cost = cost;
    }

    public int price(Map<Character, Integer> characterCounts) {
        if (!characterCounts.containsKey(this.product)) {
            return 0;
        }
        Integer foundCount = characterCounts.get(this.product);
        return this.cost * foundCount;
    }
}

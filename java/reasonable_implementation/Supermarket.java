package reasonable_implementation;

import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Supermarket implements Market {
    final private List<PricingRule> priceRules;

    public Supermarket() {
        priceRules = Arrays.asList(
                new FlatPrice('A', 20),
                new BundlePrice('B', 50, 5, 150),
                new FlatPrice('C', 30)
                );
    }

    public int checkout(String items) throws IllegalArgumentException {
        if (items == null) {
            throw new IllegalArgumentException("items argument to checkout function must not be null.");
        }
        Map<Character, Integer> counts = countCharacters(items);
        int cost = 0;
        for (PricingRule p: this.priceRules) {
            cost += p.price(counts);
        }
        return cost;
    }

    private Map<Character, Integer> countCharacters(String items) {
        HashMap<Character, Integer> count = new HashMap<Character, Integer>();
        for(char c : items.toCharArray()) {
            if (count.containsKey(c)) {
                int previous = count.get(c);
                count.put(c, previous + 1);
            } else {
                count.put(c, 1);
            }
        }
        return count;
    }
}

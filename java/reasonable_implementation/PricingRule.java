package reasonable_implementation;

import java.util.Map;

public interface PricingRule {
    int price(Map<Character, Integer> characterCounts);
}

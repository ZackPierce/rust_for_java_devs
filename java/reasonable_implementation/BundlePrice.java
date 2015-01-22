package reasonable_implementation;

import java.util.Map;

public class BundlePrice implements PricingRule {
    final private char product;
    final private int loneCost;
    final private int bundleSize;
    final private int bundleCost;

    public BundlePrice(char product, int loneCost, int bundleSize, int bundleCost) {
        this.product = product;
        this.loneCost = loneCost;
        this.bundleSize = bundleSize;
        this.bundleCost = bundleCost;
    }

    public int price(Map<Character, Integer> characterCounts) {
        if (!characterCounts.containsKey(this.product)) {
            return 0;
        }
        int count = characterCounts.get(this.product);
        if (count == 0) {
            return 0;
        }
        int bundles = count / this.bundleSize;
        int leftovers = count % this.bundleSize;
        return (bundles * this.bundleCost) + (leftovers * this.loneCost);
    }
}

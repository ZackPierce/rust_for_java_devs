package reasonable_implementation;

import org.junit.Test;

import java.util.*;

import static org.junit.Assert.assertEquals;

public class SupermarketTest {

    final private static int NUM_TEST_ITERATIONS = 1000;
    final private static int MAX_ITEMS_STRING_SIZE = 1000;

    @Test
    public void canonicalInput() {
        Supermarket s = new Supermarket();
        String items = "ABBACBBAB";
        assertEquals(240, s.checkout(items));
    }

    @Test
    public void emptyInput() {
        Supermarket s = new Supermarket();
        String items = "";
        assertEquals(0, s.checkout(items));
    }


    @Test
    public void ignoresUnrelatedItems() {
        Supermarket s = new Supermarket();
        String items = "XKD";
        assertEquals(0, s.checkout(items));
    }

    @Test
    public void mixesStandardAndUnregisteredItemPrices() {
        Supermarket s = new Supermarket();
        String items = "AXBC";
        assertEquals(100, s.checkout(items));
    }

    @Test
    public void singleBundleGetsComboPrice() {
        Supermarket s = new Supermarket();
        String items = "BBBBB";
        assertEquals(150, s.checkout(items));
    }

    @Test
    public void singleBundleWithLeftoversGivesDealPricePlusIndividual() {
        Supermarket s = new Supermarket();
        String items = "BBBBB B";
        assertEquals(200, s.checkout(items));
    }

    @Test
    public void multipleBundlesEachGetDealPricePlusLeftovers() {
        Supermarket s = new Supermarket();
        String items = "BBBBB BBBBB BB";
        assertEquals(400, s.checkout(items));
    }

    private static class StringWithSingleCharacterCount {
        String items;
        int len;

        StringWithSingleCharacterCount(String items, int len) {
            this.items = items;
            this.len = len;
        }
    }

    private static StringWithSingleCharacterCount generateCharacterSequence(char c) {
        Random rng = new Random();
        int n = rng.nextInt(MAX_ITEMS_STRING_SIZE - 1) + 1;
        StringBuilder seq = new StringBuilder();
        for (int i = 0; i < n; i++) {
            seq.append(c);
        }
        return new StringWithSingleCharacterCount(seq.toString(), n);
    }

    @Test
    public void correctlySumsSequencesOfManySizesOfAs() {
        Supermarket s = new Supermarket();
        for (int i = 0; i < NUM_TEST_ITERATIONS; i++) {
            StringWithSingleCharacterCount seqWithCount = generateCharacterSequence('A');
            assertEquals(seqWithCount.len * 20, s.checkout(seqWithCount.items));
        }
    }

    @Test
    public void correctlySumsSequencesOfManySizesOfBs() {
        Supermarket s = new Supermarket();
        for (int i = 0; i < NUM_TEST_ITERATIONS; i++) {
            StringWithSingleCharacterCount seqWithCount = generateCharacterSequence('B');
            assertEquals(((seqWithCount.len / 5) * 150) + ((seqWithCount.len % 5) * 50), s.checkout(seqWithCount.items));
        }
    }

    @Test
    public void correctlySumsSequencesOfManySizesOfCs() {
        Supermarket s = new Supermarket();
        for (int i = 0; i < NUM_TEST_ITERATIONS; i++) {
            StringWithSingleCharacterCount seqWithCount = generateCharacterSequence('C');
            assertEquals(seqWithCount.len * 30, s.checkout(seqWithCount.items));
        }
    }

    static class StringWithCharacterCounts {
        String items;
        Map<Character, Integer> counts;

        private StringWithCharacterCounts(String items, Map<Character, Integer> counts) {
            this.items = items;
            this.counts = counts;
        }
    }

    private static StringWithCharacterCounts generateMixedCharSequence(List<Character> chars) {
        Random rng = new Random();
        int n = rng.nextInt(MAX_ITEMS_STRING_SIZE - 1) + 1;
        int numChoices = chars.size();
        Map<Character, Integer> count = new HashMap<Character, Integer>();
        StringBuilder s = new StringBuilder();
        for (int i = 0; i < n; i++) {
            Character c = chars.get(rng.nextInt(numChoices));
            s.append(c);
            if (count.containsKey(c)) {
                count.put(c, count.get(c) + 1);
            } else {
                count.put(c, 1);
            }
        }
        return new StringWithCharacterCounts(s.toString(), count);
    }

    private static int simpleExpectedPrice(Map<Character, Integer> counts) {
        int aCost = 0;
        if (counts.containsKey('A')) {
            aCost = counts.get('A') * 20;
        }

        int bCost = 0;
        if (counts.containsKey('B')) {
            int numBs = counts.get('B');
            bCost = ((numBs / 5) * 150) + ((numBs % 5) * 50);
        }

        int cCost = 0;
        if (counts.containsKey('C')) {
            cCost = counts.get('C') * 30;
        }

        return aCost + bCost + cCost;
    }

    @Test
    public void correctlySumsRandomSequenceOfValidCodes() {
        List<Character> standardCodes = Arrays.asList('A', 'B', 'C');
        Supermarket s = new Supermarket();
        for (int i = 0; i < NUM_TEST_ITERATIONS; i++) {
            StringWithCharacterCounts seqWithCounts = generateMixedCharSequence(standardCodes);
            assertEquals(simpleExpectedPrice(seqWithCounts.counts), s.checkout(seqWithCounts.items));
        }
    }
}
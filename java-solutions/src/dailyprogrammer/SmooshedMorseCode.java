package dailyprogrammer;

import java.io.BufferedReader;
import java.io.FileReader;
import java.util.*;
import java.util.stream.Collectors;

/// in this challenge morse code dots and dashes will be 'smooshed' together. Write a function:
/// `snorse` that takes a string of letters as input, and outputs morse code dots and dashes
/// smooshed together
///
/// # Example
///
/// ```
/// smorse("sos") => "...---..."
/// smorse("daily") => "-...-...-..-.--"
/// smorse("programmer") => ".--..-.-----..-..-----..-."
/// smorse("bits") => "-.....-..."
/// smorse("three") => "-.....-..."
/// ```
///
/// An obvious problem with this system is that decoding is ambiguous. For instance,
/// both `bits` and `three` encode to the same string, so you can't tell which one you would
/// decode to without more information.
///
/// #  Bonus Challenges
/// 1. The sequence `-...-....-.--.` is the code for four different words
///   `(needing, nervate, niding, tiling)`. Find the only sequence that's the code
///   for 13 different words.


public class SmooshedMorseCode {

    private HashMap<String, String> morseMap;

    public SmooshedMorseCode() {
        this.morseMap = buildMorseMap();
    }

    private HashMap<String,String> buildMorseMap() {
        HashMap<String,String> map = new HashMap<>(26);
        String CODES = ".- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -. --- .--. --.- .-. ... - ..- ...- .-- -..- -.-- --..";
        List<String> codes = Arrays.asList(CODES.split(" "));
        char letter = 'a';
        for (int i = 0; i < 26; i++) {
            map.put(String.valueOf(letter++), codes.get(i));
        }
        return map;
    }

    public String smorse(String str) {
        return Arrays
                .stream(str.split(""))
                .map(c -> this.morseMap.get(c))
                .collect(Collectors.joining());
    }

    // map a list of words from the file located at 'path' into a HashMap of morseCode -> List<word>
    public Map<String, List<String>> mapFile(String path) {
        Map<String,List<String>> codeMap = new HashMap<>();
        try (BufferedReader br = new BufferedReader(new FileReader(path))) {
            String line;
            while ((line = br.readLine()) != null) {
                String smorse = this.smorse(line);
                if (codeMap.containsKey(smorse)) {
                    codeMap.get(smorse).add(line);
                } else {
                    List<String> words = new ArrayList<>();
                    words.add(line);
                    codeMap.put(smorse, words);
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return codeMap;
    }


    public static void main(String[] args) {
        SmooshedMorseCode smc = new SmooshedMorseCode();
        String path = "./java-solutions/src/dailyprogrammer/enable1.txt";
        //System.out.println( String.format("sos = %s", smc.smorse("sos")));

        Map<String, List<String>> mappings = smc.mapFile(path);
        // BONUS, find the code that maps to 13 words
        final Optional<Map.Entry<String, List<String>>> oentry = mappings.entrySet().stream().filter(es -> es.getValue().size() == 13).findFirst();
        if (oentry.isPresent()) {
            System.out.println("found 13 entries for code " + oentry.get().getKey() + " with words " + oentry.get().getValue());
        } else {
            System.out.println("no code that maps to the same 13 words");
        }
    }
}

package euler;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 * # Problem 22 - Names Scores
 * Using `names.txt`, a 46K text file containing over five-thousand first names, begin by
 * sorting it into alphabetical order. Then working out the alphabetical value for each name,
 * multiply this value by its alphabetical position in the list to obtain a name score.
 * For example, when the list is sorted into alphabetical order, COLIN, which is worth:
 * 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score
 * of 938 × 53 = 49714.
 * What is the total of all the name scores in the file?
 */
public class P22NamesScores {

    static final String FILE_PATH = "./euler-java/src/p022_names.txt";

    static List<String> parseFile() {
        List<String> resNames = new ArrayList<>();

        try (Stream<String> stream = Files.lines(Paths.get(FILE_PATH))) {

            stream.forEach(line -> {
                List<String> cleanNames =  Arrays.stream(line.split(","))
                        .map(name -> name.replace("\"", ""))
                        .collect(Collectors.toList());
                resNames.addAll(cleanNames);
            });

        } catch (IOException e) {
            e.printStackTrace();
        }
        return resNames;
    }

    // map each character in name to it's ordinal position in the alphabet from A-Z
    // ex. "ABC" = 1 + 2 + 3 = 6
    static int nameScore(String name) {
        return name.chars().map(ch -> ch - 65 + 1).sum();
    }

    public static void main(String[] args) {
        List<String> names = parseFile();
        names.sort(Comparator.naturalOrder());

        long totalScore = 0;
        for (int idx = 0; idx < names.size(); idx++) {
            totalScore += nameScore(names.get(idx)) * (idx + 1);
        }
        // 871198282
        System.out.println(String.format("total score is %d", totalScore));
    }
}

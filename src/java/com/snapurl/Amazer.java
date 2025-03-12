package com.snapurl;
import java.net.URL;
import java.io.IOException;
import com.beegman.webbreaker.WebSession;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class Amazer {
    public static void main(String...args) {
        var session = new WebSession().create();
        try {
            var page = session.read(new URL("https://liliputing.com/orange-pi-rv2-is-a-single-board-pc-with-an-8-core-risc-v-processor/"), 0);
            int randomValue = ThreadLocalRandom.current().nextInt(0, 255); // --> get max from config
            Files.write(Paths.get(args[0], ".fak", String.valueOf(randomValue) + ".scr"), page.getText());
        } catch(IOException mue) {
            System.err.printf("the URL reading error - %s%n", mue);
            mue.printStackTrace();
        }
    }
}
package com.snapurl;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import org.jsoup.Jsoup;
import org.jsoup.nodes.Document;
import org.jsoup.nodes.Element;
import org.jsoup.select.Elements;
import java.util.concurrent.ThreadLocalRandom;

public class Amazer {
    public static void main(String...args) {
        var ourl = "https://liliputing.com/orange-pi-rv2-is-a-single-board-pc-with-an-8-core-risc-v-processor/";
        String burl = "https://stackoverflow.com";
        try {
            Document document = Jsoup.connect(burl+"/questions").get();
            //System.out.println("Document..."+document);
            // Select the elements that contain the question titles
            Elements questionElements = document.select(".s-post-summary--content .s-link");

            // Print the titles of the questions
            for (Element questionElement : questionElements) {
                String questionTitle = questionElement.text();
                String hurl = questionElement.attr("href");
                System.out.printf("%s at %s%n", questionTitle, hurl );
                document = Jsoup.connect("https://stackoverflow.com"+hurl).get();
                //System.out.println("Document..."+document);
                var randomValue = ThreadLocalRandom.current().nextInt(0, 255); // --> get max from config
                var content = new ArrayList<String>(390);
                content.add("<h2>"+questionTitle+"</h2>");
                //content.addAll(document.select(".s-prose.js-post-body"));
                for(var qu2l: document.select(".s-prose.js-post-body")) {
                      content.add(qu2l.toString());
               
                }
                Files.write(Paths.get(args[0], ".fak", String.valueOf(randomValue) + ".scr"), content);
            }
        } catch(IOException mue) {
            System.err.printf("the URL reading error - %s%n", mue);
            mue.printStackTrace();
        }
    }
}
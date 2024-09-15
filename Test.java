
/**
 * Test
 */
public class Test {

  public static void main(String[] args) throws InterruptedException {
    System.out.println("Hello World");

    var thread = new Thread(() -> {
      System.out.println("Thread is running");
    });

    thread.start();
    thread.join();
  }
}

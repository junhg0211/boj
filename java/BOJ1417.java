import java.util.Scanner;

public class BOJ1417 {
    static int[] numbers = new int[50];
    static int count;

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        count = scanner.nextInt();

        int max = 0;
        numbers[0] = scanner.nextInt();
        for (int i = 1; i < count; i++) {
            numbers[i] = scanner.nextInt();
            max = Math.max(max, numbers[i]);
        }

        if (max < numbers[0]) {
            System.out.println("0");
            return;
        }

        int gets;
        for (int other = max; ; other--) {
            gets = 0;
            for (int i = 1; i < count; i++) {
                gets += Math.max(0, numbers[i] - other);
            }

            if (numbers[0] + gets - other == 1) {
                System.out.println(gets);
                break;
            }

            if (numbers[0] + gets > other) {
                System.out.println(other + 2 - numbers[0]);
                break;
            }
        }

        scanner.close();
    }
}

package leetcode;

public class P7ReverseNumber {
    public int reverse(int x) {
        long rev = 0;
        while(x != 0){
            rev = rev*10 + x%10;
            x /= 10;
        }
        System.out.println("rev is = " + rev);
        System.out.println("max int= " + Integer.MAX_VALUE);
        return (rev<=Integer.MAX_VALUE && rev>= Integer.MIN_VALUE)?(int)rev:0;
    }

    public static void main(String[] args) {
        var p7 = new P7ReverseNumber();
        var r = p7.reverse(1_534_236_469);
        System.out.println(r);
    }
}

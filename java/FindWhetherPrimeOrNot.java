/******************************************************************************

 Java Program  for cerner_2^5_2019

 *******************************************************************************/
package java;

import java.util.Scanner;
import java.lang.Math;

public class FindWhetherPrimeOrNot {

    public static void main(String args[]){
        Scanner scan = new Scanner(System.in);
        long n;
        int flg = 0;

        System.out.println("Enter a number"); // accept number from user
        n = scan.nextLong();

        /*
         loop from 2 till the number and check if the number is user input is divisible by any number
         */
        for(int i = 2; i < Math.sqrt((double)n);i++){
            if(n % i == 0){
                flg = 1;
                break;
            }
        }

        /*
         Based on the flag set in the above loop, print whether the number is prime or not
         */
        if(flg == 0){
            System.out.println("Prime");
        }
        else{
            System.out.println("Not a Prime");
        }
    }
}

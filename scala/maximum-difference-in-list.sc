/******************************************************************************

 Scala Program for cerner_2^5_2019

 *******************************************************************************/
object FindMaximumDifferenceInList {
    def main(args: Array[String]) {
        print(List(2,5,10,6,4,8,1).combinations(2).map{case List(a,b) => b-a}.max); // finding maximum difference between 2 elements in a list
    }
}
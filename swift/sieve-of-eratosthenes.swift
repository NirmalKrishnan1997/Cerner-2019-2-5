/******************************************************************************

 Swift Program for cerner_2^5_2019

 *******************************************************************************/
func primeNumbersList(lessThan:Int)->[Int]{
    var primeNumbers:[Int] = []
    //Create list of integers from 2...lessThan
    var allNumbers = Array(2...lessThan)
    var allNumTuple = [(name: Int, value: Bool)]()
    for num in allNumbers{
        allNumTuple.append((name:num, value:true))
    }
    var index = 2
    while index < lessThan{
        if allNumTuple[index].value == false{
            index += 1
            continue
        }
        var iCount = Int(lessThan/index)
        if iCount == 1{
            break
        }
        for i in 2...iCount{
            var refIndex = i * index
            if (refIndex <= lessThan){
                    allNumTuple[(refIndex-2)].value = false
            }
            
        }
        index += 1
    }
    var nums = allNumTuple.filter { (name:Int, value:Bool) -> Bool in
        return value == true
    }
    for item in nums{
        primeNumbers.append(item.name)
    }
    return primeNumbers
}

var primeNumbers = primeNumbersList(lessThan: 100)
print("prime numbers less than 100 = \(primeNumbers)")
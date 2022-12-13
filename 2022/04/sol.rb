sol1 = File.read('input.txt')
  .split("\n")
  .map{ |e| e.split(',').map{ |i| i.split('-').map(&:to_i) } }
  .filter{ |e| 
    (e[0][0] <= e[1][0] and e[0][1] >= e[1][1]) or
    (e[1][0] <= e[0][0] and e[1][1] >= e[0][1]) 
  }
  .count
  
print "Solution 1: #{sol1}"

sol2 = File.read('input.txt')
  .split("\n")
  .map{ |e| e.split(',').map{ |i| i.split('-').map(&:to_i) } }
  .filter{ |e| not (e[0][0]..e[0][1]).to_a.intersection((e[1][0]..e[1][1]).to_a).empty? }
  .count
  
print "Solution 2: #{sol2}"
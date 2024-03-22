### 0.1.0
I am using the most basic approach in this first iteration by reading the file using BufReader and storing the measurements in a hashmap. Then calculate the min,mean and max for each station and store it in a struct, loop over that struct and print each value. I am using a buffer reader.

### 0.2.0
The previous method was taking too long (>1 hr), probably because i was storing all the measurements in an array and looping over them. I have changed the implemtation to store the initial value as the min, max and mean, and compare each value it reads and replace if applicable. This brought the execution time down to around 23min.

### 0.3.0
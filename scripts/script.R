install.packages("rjson")
library("rjson")
json_data <- fromJSON();
df <- lapply(my.JSON, function(play) # Loop through each "play"
  {
  # Convert each group to a data frame.
  # This assumes you have 6 elements each time
  data.frame(matrix(unlist(play), ncol=6, byrow=T))
  })
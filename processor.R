library("RPostgreSQL")
library("shiny")
library(hash)
library(mosaic)

conn <- dbConnect(drv=PostgreSQL(), 
                  user="cardinal", 
                  password="", 
                  host="localhost", 
                  port=5432,
                  dbname="myhealth")

ui <- fluidPage(
  titlePanel("mobile stats analyzer "),
    fluidRow(
      column(
        width = 3,
        textInput(inputId = "userid", label = "participant_id", placeholder = "participannt id"),
        actionButton(inputId = "idsubmit", label = "submit")
      )
    ),
    fluidRow(
      column(
        width = 3,
        h3("App Usage Analysis"),
        plotOutput("plot", height=300),
        verbatimTextOutput("value")
      ),
      column(
        width = 3,
        h3("SMS annalysis"),
        plotOutput("plot2", height=300)
      )
    )
)

append_query_num <- function(query, value) {
  new_query <- paste(query, value)
  return(new_query)
}

append_query_str <- function(query, value) {
  print(value)
  new_query <- paste(query, "'", value, "'", sep="")
  return(new_query)
}

server <- function(input, output, session) {
  observeEvent(
    eventExpr = input[["idsubmit"]],
    handlerExpr = {
      id <- initialize(input$userid)
      print(id)
      output$plot <- renderPlot ({ d <- appUsageAnalyze(conn, id) })
      
      output$value <- renderPrint({ favstats(smsAnalyze(conn, id)) })    
    }
  )
}

toDate <- function(dates) {
  newDates <- c();
  for (date in dates) {
    newDate <- as.Date(as.POSIXct(date / 1000, origin="1970-01-01"), format="%m/%d/%Y")
    
    newDates <- append(newDates, newDate)
  }
  
  return(newDates)
}

dayCount <- function(dates) {
  counts <- c()
  # make sure the dates are sorted
  # dates <- order(as.Date(dates, format = "%m/%d/%Y"))
  
  # find the first day
  currentDate <- ""
  # first count
  count <- 0
  for(date in dates) {
    
    if(date != currentDate) {
      counts <- append(counts, count)
      count <- 1
      currentDate = date
    }else{
      count <- count + 1
    }
  }
  return(counts)
}

smsAnalyze <- function(conn, id) {
  query <- append_query_str("SELECT date from smsdata where participant_id=", id) 
  smsdates <- dbGetQuery(conn, query)
  dates <- toDate(smsdates)
  
  freq <- dayCount(dates)
}

# this simply counts each app event, putting it into a named list of events, it doesn't take count of time
procAppEvents <- function(dbframe) {
  events <- hash()
  
  for(event in dbframe$package_name) {
    # need to append each event's information to a vector assigned to the value of the package_name
    
    if(is.null(events[[event]])) {
      # events <- append(events, event$package_name)
      events[[event]] <- 0
    }
    events[[event]] <- events[[event]] + 1
  }
  
  return(events)
}

greater_than <- function(v1, v2) {
  if(v1 > v2) {
    return(TRUE)
  }else{
    return(FALSE)
  }
}

keep <- function(data, value, condition) {
  newvals <- hash()
  for(element in names(data)) {
    if(!is.null(data[[element]]) && condition(data[[element]], value) == TRUE) {
      newvals[[element]] <- data[[element]]
    }
  }
  return(newvals)
}

appUsageAnalyze <- function(conn, id) {
  query <- append_query_str("SELECT package_name,date from appusageevents where participant_id=", id)
  appevents <- dbGetQuery(conn, query)
  totals <- procAppEvents(appevents)
  m <- mean(values(totals))
  print(m)
  kept <- keep(totals, m, greater_than)
  print(kept)
  return(pie(values(kept)))
}

initialize <- function(user_id){
  query <- append_query_num("SELECT id FROM participants WHERE participant_id = ", user_id)
  print(query)
  squery <- dbSendQuery(conn, query)
  result <- dbFetch(squery)
  return(result$id)
}

shinyApp(ui, server, onStop(function() {
  dbDisconnect(conn)
}))
dbDisconnect(conn)
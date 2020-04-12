+ read file & stdin in similar manner and output it to stdout
- convert each line of read data to an ordered hash table based on regex
- parse query to AST
- execute AST


Components
  - input: reads the textual stream and parses it to a stream of samples (hashtables)
  - query (scatterer): performs transformation on the stream of samples producing a new stream
  - output (gatherer): prints out samples in a specified format (InfluxDB, JSON, OpenTSB, etc)
  - [optional] console UI: show/edit input, queries, and visualize outputs (graph, histogram, tabular)

A time series = stream name + set of labels (icluding their values).
Queries produce one or more time series.



Prometheus vs IndluxDB
sample        point
timestamp     timestamp
value         field
label         tag
metric        measurement
series        series

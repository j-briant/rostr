# Rostr

Minimalist CLI for managing data transfers between geospatialized formats using GDAL drivers capabilities.

## Objectives

Clearly not to build a full fledged ETL:

* Clearly define sources, destinations and options.
* "Transform" data using (GDAL flavored) sql.
* Have a simple interface to add, remove, update and query jobs.
* Make use of multithreading capabilities when required.

And that's basically it.

Most of the job resides in a coherent configuration management.

For people unfortunately too familiar with FME, if most of the jobs are data translation then one of the objective is to give an alternative.

## Misc

It's early af to be here.
[![Build & Test](https://github.com/billyto/week/actions/workflows/build.yml/badge.svg)](https://github.com/billyto/week/actions/workflows/build.yml)


# week
Simple CLI utility to return the ISO week number starting from 1.

## Use cases

```
week  //The week of the year for today

week [--date date] //Week for a given 'date'
```

## Suported Date formats
* %d-%m-%Y
* %d/%m/%Y
* %d.%m%.%Y
* %d-%m
* %d/%m
* %d.%m


---

### TODOs:

[x] ~~fix valid formats, no yeat first and right %xx descriptors~~
[x] ~~CICD for release version~~
[] Better cli args descriptions
[x] ~~Tests on ErrorParse~~  

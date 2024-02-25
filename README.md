# week
Simple CLI utility to return the ISO week number starting from 1.

## Use cases

```
week  //The week of the year for today

week [--date date] //Gives the week for 'date'
```

## Suported Date formats
* %d-%m-%Y

* %d/%m/%Y

* %d/%m

* %d.%m


---

### TODOs:

[x] fix valid formats, no yeat first and right %xx descriptors

[x] CICD for release version

[] Better cli args descriptions

[] Tests on ErrorParse  

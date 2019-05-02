## Build and launch docker container
```bash
$ sh build.sh
$ sh run.sh
```

## Extract wikipedia texts
```bash
# Inside docker container
$ python wikiextractor/WikiExtractor.py -b 500M -o ./data/wikipedia/20190502/ \
  ./data/wikipedia/20190502/jawiki-latest-pages-articles.xml.bz2
```

Result:
```bash
INFO: Finished 3-process extraction of 1148622 articles in 2310.4s (497.2 art/s)
INFO: total of page: 1652185, total of articl page: 1148622; total of used articl page: 1148622
```

Output files:
```bash
$ ls ~/data/wikipedia/20190502/
AA                                      jawiki-latest-pages-articles.xml.bz2
$ ls -lh ~/data/wikipedia/20190502/AA
total 5770304
-rw-r--r--  1 taku-y  staff   500M May  2 22:03 wiki_00
-rw-r--r--  1 taku-y  staff   500M May  2 22:10 wiki_01
-rw-r--r--  1 taku-y  staff   500M May  2 22:18 wiki_02
-rw-r--r--  1 taku-y  staff   500M May  2 22:24 wiki_03
-rw-r--r--  1 taku-y  staff   500M May  2 22:31 wiki_04
-rw-r--r--  1 taku-y  staff   245M May  2 22:36 wiki_05
```

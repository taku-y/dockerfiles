import codecs
import re
import MeCab
import neologdn


class WikipediaIterator:
    def __init__(self, files):
        self.files = files
        self.m = MeCab.Tagger('-Owakati')

    def __iter__(self):
        # Loop over files
        for file in self.files:

            # Open file as UTF-8
            with codecs.open(file, "r", "utf_8") as f:

                # Loop over lines in a file
                for line in iter(f.readline, ''):
                    if line.startswith("<doc"):
                        # Skip next line
                        f.readline()
                        continue
                    elif line.startswith("</doc>"):
                        continue
                    elif line == "\n":
                        continue
                    else:
                        for s in line.strip("\n").split("。")[:-1]:
                            # Regarding text normalization, see the below link:
                            # https://ohke.hateblo.jp/entry/2019/02/09/141500
                            s = neologdn.normalize(s)
                            s = re.sub(r'[!-/:-@[-`{-~]', r' ', s)
                            s = re.sub(u'[■-♯]', ' ', s)
                            s = re.sub(r'\d+', '0', s)

                            # Split sentense into tokens
                            s = self.m.parse(s).strip("\n").split(" ")[:-1]
                            
                            # Return a list of tokens
                            yield s


if __name__ == "__main__":
    import io,sys
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

    files = ["/root/data/wikipedia/20190502/AA/wiki_00",
             "/root/data/wikipedia/20190502/AA/wiki_01",
             "/root/data/wikipedia/20190502/AA/wiki_02",
             "/root/data/wikipedia/20190502/AA/wiki_03",
             "/root/data/wikipedia/20190502/AA/wiki_04",]

    doc = WikipediaIterator(files)

    for token in doc:
        input(token)


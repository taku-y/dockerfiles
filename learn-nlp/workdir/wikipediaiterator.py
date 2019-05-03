import codecs
import re
import MeCab
import neologdn


class WikipediaIterator:
    def __init__(self, files, max_sentenses=0):
        self.files = files
        self.m = MeCab.Tagger('-Owakati')
        self.max_sentenses = max_sentenses

    def __iter__(self):
        count = 0

        # Loop over files
        for file in self.files:

            # Open file as UTF-8
            with codecs.open(file, "r", "utf_8") as f:

                # Loop over lines in a file
                for line in iter(f.readline, ''):
                    # Skip some lines
                    if line.startswith("<doc"):
                        # Skip next line
                        f.readline()
                        continue
                    elif line.startswith("</doc>"):
                        continue
                    elif line == "\n":
                        continue
                    
                    for s in line.strip("\n").split("。")[:-1]:
                        # Regarding text normalization, see the below link:
                        # https://ohke.hateblo.jp/entry/2019/02/09/141500
                        s = neologdn.normalize(s)
                        s = re.sub(r'[!-/:-@[-`{-~]', r' ', s)
                        s = re.sub(u'[■-♯]', ' ', s)
                        s = re.sub(r'(\d)([,.])(\d+)', r'\1\3', s)
                        s = re.sub(r'\d+', '0', s)

                        # Split sentense into tokens
                        s = self.m.parse(s).strip("\n").split(" ")[:-1]
                        
                        # Return a list of tokens
                        count += 1
                        if count == self.max_sentenses:
                            raise StopIteration()
                        else:
                            yield s


if __name__ == "__main__":
    import io,sys
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

    files = ["/root/data/wikipedia/20190502/AA/wiki_00"] #,
             # "/root/data/wikipedia/20190502/AA/wiki_01",
             # "/root/data/wikipedia/20190502/AA/wiki_02",
             # "/root/data/wikipedia/20190502/AA/wiki_03",
             # "/root/data/wikipedia/20190502/AA/wiki_04",]

    from gensim.models.fasttext import FastText
    from time import time
    dim_word_vec = 10
    window = 3 # words
    max_sentenses = 0 # 0 for all sentenses
    model = FastText(size=dim_word_vec, window=window, min_count=20, 
                     min_n=1, max_n=6)

    print("=== Build vocaburuary ===")
    t_start = time()
    wikipedia = WikipediaIterator(files, max_sentenses=max_sentenses)
    model.build_vocab(sentences=wikipedia)
    print(model)
    print("Elapsed time: {}\n".format(time() - t_start))

    print("=== Train model ===")
    t_start = time()
    wikipedia = WikipediaIterator(files, max_sentenses=max_sentenses)
    model.train(sentences=wikipedia, total_examples=model.corpus_count,
                total_words=model.corpus_total_words, epochs=2)
    print(model)
    print("Elapsed time: {}\n".format(time() - t_start))

    print("=== Save model ===")
    model.save("saved_model_gensim")

import io,sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

import MeCab
m = MeCab.Tagger('-Owakati')
analyzed = m.parse('我輩は猫である。名前はまだ無い。')
print(analyzed)
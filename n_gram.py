
import MeCab

with open('text.txt', mode='rt', encoding='utf-8') as f:
    read_text = f.read()
nekotxt = read_text

print(nekotxt)

tagger = MeCab.Tagger("-Owakati")

#分かち書き
#['xxx', 'xxx', 'xxx',...]
parsed_txt = tagger.parse(nekotxt).split()


from collections import Counter
import numpy as np
from numpy.random import *

delimiter = ['「','」','…','　']

#2語のリスト
double = list(zip(parsed_txt[: -1], parsed_txt[1:]))
double = filter(
    (lambda x: not((x[0] in delimiter) or (x[1] in delimiter))),
    double
)

#要素数をカウントして辞書を作成
count_dic = Counter(double)
for u, v in count_dic.items():
    print(u, v)



from flair.data import Sentence
from flair.models import SequenceTagger
from flair.data import LabelDictionary

# load tagger
tagger = SequenceTagger.load("flair/ner-english-fast")

labels = ["CUSTOM_LABEL1", "CUSTOM_LABEL2"]
# Remove the existing labels
tagger.label_dictionary = LabelDictionary(labels)


# make example sentence
sentence = Sentence("George Washington went to Washington")

# predict NER tags
tagger.predict(sentence)

# print sentence
print(sentence)

# print predicted NER spans
print('The following NER tags are found:')
# iterate over entities and print
for entity in sentence.get_spans('ner'):
    print(entity)

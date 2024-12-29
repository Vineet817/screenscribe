from transformers import MBartForConditionalGeneration, MBart50TokenizerFast
import sys

def translate_text(text, source_lang, target_lang):
    model = MBartForConditionalGeneration.from_pretrained("facebook/mbart-large-50-many-to-many-mmt")
    tokenizer = MBart50TokenizerFast.from_pretrained("facebook/mbart-large-50-many-to-many-mmt")

    tokenizer.src_lang = source_lang
    tokenizer.tgt_lang = target_lang

    inputs = tokenizer(text, return_tensors="pt")
    translated_tokens = model.generate(**inputs, forced_bos_token_id=tokenizer.lang_code_to_id[target_lang])
    translated_text = tokenizer.batch_decode(translated_tokens, skip_special_tokens=True)[0]

    return translated_text

if __name__ == "__main__":
    text = sys.argv[1]
    source_lang = sys.argv[2]
    target_lang = sys.argv[3]
    print(translate_text(text, source_lang, target_lang))

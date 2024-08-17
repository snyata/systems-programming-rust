'''
PYTHON SCRIPT TO CALL
---------------------


'''
import sys
import json
from llama_index import GPTIndex, SimpleDocument

#A Function for processing a batch of data through llama index

def process_batch(input_data: json) -> json:
    index: GPTIndex = GPTIndex()
    
    output_data = []
    for data in input_data:
        doc = SimpleDocument(text=data['user_prompt'])
        response = index.query(doc)
        output_data.append({"id": data["id"], "output_text": response})

    return output_data


if __name__ == "__main__":
    if input_data > 0:
        process_batch(input_data={"id": 0, "user_prompt": "empty json"})
    else:
        raise ValueError("Invalid input data")
        sys.exit(1)
    
import requests
import pandas as pd

from enum import Enum
from pathlib import Path

# /************************************************************/
class GeonameTable:
    '''
    '''
    def __init__(self, table_type: str = None, save_parent: Path = None):
        '''
        '''
        self.url = "https://download.geonames.org/export/dump/"
        match table_type: 
            case "nations":
                self.url += "countryInfo.txt"
            case "regions":
                self.url += "admin1CodesASCII.txt"
            case "cities":
                self.url += "cities500.zip"
            case _: 
                raise NotImplementedError(f'Table Type "{table_type}"')
    
    def _strip(self, data: str) -> str: 
        lines = data.decode('utf-8').split('\n')
        header = next(line for line in reversed(lines) if line.startswith('#')).lstrip('#')
        clean_data = '\n'.join(line for line in lines if not line.startswith('#'))
        return header, clean_data

    def fetch(self):

        data = requests.get(self.url).content
        header, data = self._strip(data)
        data = header + '\n' + data
 
        df = pd.read_csv(data, sep="\t", header=true) 
        return df







# /************************************************************/
if __name__ == "__main__":
    p = GeonameTable("nations")
    df = p.fetch()
    print(df)
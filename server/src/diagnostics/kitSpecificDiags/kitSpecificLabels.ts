import { kits } from '../../constants/kits';
import { debug } from '../../debug';
import { getGlobalSettings } from '../../server'
import { dsm51 } from './dsm51/labels';

export const getKitLabels = () : Array<string> => {
	let kit = getGlobalSettings().kit;
	debug.info(`selected kit: ${kit}\n settings:`);
	debug.info(getGlobalSettings());

	if(kit == kits.dsm51) return dsm51.labels;

	return [];
}
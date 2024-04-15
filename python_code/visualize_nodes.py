import re
import pyvis

# Regex pattern to match the line format
pattern = re.compile(r"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$")
FONT_SIZE = 50

def get_node_color(node: str) -> str:
    match node[-1]:
        case 'Z':
            return 'red'
        case 'A':
            return 'blue'
        case _:
            return 'green'

def create_font_data(node: str) -> dict:
    return {
        'size': FONT_SIZE,
        'color': get_node_color(node),
    }

def add_node_to_network(network: pyvis.network.Network, node: str) -> None:
    color = get_node_color(node)
    font_data = create_font_data(node)
    network.add_node(node, label=node, color=color, font=font_data)

def add_edge_to_network(network: pyvis.network.Network, start_node: str, end_node: str) -> None:
    network.add_edge(start_node, end_node)

def create_network_from_input_file(input_file_path: str) -> pyvis.network.Network:
    network = pyvis.network.Network(height="600px", width="800px", bgcolor="#2")
    with open(input_file_path) as file:
        for line in file:
            if not (match := pattern.match(line)):
                continue

            node = match.group(1)
            add_node_to_network(network, node)

            left = match.group(2)
            add_node_to_network(network, left)

            right = match.group(3)
            add_node_to_network(network, right)

            add_edge_to_network(network, node, left)
            add_edge_to_network(network, node, right)

    return network

def show_network_in_browser(network: pyvis.network.Network, output_file_path: str, notebook: bool) -> None:
    network.show(output_file_path, notebook=notebook)

def main() -> None:
    input_file_path = r"D:\git\adventofcode2023\inputs\8\input.txt"
    output_file_path = "graph.html"
    network = create_network_from_input_file(input_file_path)
    show_network_in_browser(network, output_file_path, notebook=False)

if __name__ == "__main__":
    main()
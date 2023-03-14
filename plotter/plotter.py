import matplotlib.pyplot as plt
import pandas as pd
import plotly.express as px
import seaborn as sns

# plot coming from article
# https://towardsdatascience.com/9-visualizations-to-show-proportions-or-percentages-instead-of-a-pie-chart-4e8d81617451
# Data types and data structures used in the Rust compiler
"""
Hashmap : {
    String: 18613,
    Fixed size Array: 9291,
    Numbers: 304618,
    Struct: 48955,
    Keyword unsafe: 9693,
    Dynamic vectors: 14278,
    Heap allocation Box Rc or Arc: 6971,
    Enum: 15040,
}
Total lines : 23207248
Total number of files : 40145
"""

# Data types and data structures used in the 3 most popular crates on crates.io
"""
Hashmap : {
    String: 304,
    Fixed size Array: 1884,
    Numbers: 43525,
    Struct: 4591,
    Keyword unsafe: 209,
    Dynamic vectors: 371,
    Heap allocation Box Rc or Arc: 149,
    Enum: 581,
}
Total lines : 1742008
Total number of files : 623
"""

# Data types and data structures used in the most popular crates on lib.rs (classed by quality)
"""
Hashmap : {
    String: 1095,
    Fixed size Array: 122,
    Numbers: 11827,
    Struct: 2106,
    Keyword unsafe: 10,
    Dynamic vectors: 964,
    Heap allocation Box Rc or Arc: 97,
    Enum: 1056,
}
Total lines : 1274048
Total number of files : 985
"""

rustc = {
    "String": 18613,
    "Array": 9291,
    "Numbers": 304618,
    "Struct": 48955,
    "Vec": 14278,
    "Box/Rc/Arc": 6971,
    "Enum": 15040,
    # "Total": 40145,
}

crates_io = {
    "String": 304,
    "Array": 1884,
    "Numbers": 43525,
    "Struct": 4591,
    "Dynamic vectors": 371,
    "Box Rc or Arc": 149,
    "Enum": 581,
    # "Total": 623,
}

lib_rs = {
    "String": 1095,
    "Array": 122,
    "Numbers": 11827,
    "Struct": 2106,
    "Vec": 964,
    "Box/Rc/Arc": 97,
    "Enum": 1056,
    # "Total": 985,
}

datasets = [lib_rs, crates_io, rustc]

for d in datasets:
    # Get the name of the current dictionary object
    dict_name = next(key for key, value in locals().items() if value is d)
    df = pd.DataFrame.from_dict(d, orient="index", columns=["Value"])
    df.reset_index(inplace=True)

    df.rename(columns={"index": "Category"}, inplace=True)
    total = df["Value"].sum()
    df["Percent"] = 100 * df["Value"] / total

    # Sort the data frame by percent in descending order
    df = df.sort_values(by="Percent", ascending=False)

    df["Y"] = [1] * len(df)
    list_x = list(range(0, len(df)))
    df["X"] = list_x
    print(df)

    label = [
        i + "<br>" + str(j) + "<br>" + "{:.2f}%".format(k)
        for i, j, k in zip(df.Category, df.Value, df.Percent)
    ]

    pal_ = list(
        sns.color_palette(palette="plasma_r", n_colors=len(df.Category)).as_hex()
    )
    fig = px.scatter(
        df,
        x="X",
        y="Y",
        color="Category",
        color_discrete_sequence=pal_,
        size="Value",
        text=label,
        size_max=180,
    )
    fig.update_layout(
        width=1600, height=600, margin=dict(t=50, l=0, r=0, b=0), showlegend=False
    )

    fig.update_traces(
        textposition="top center",
        textfont=dict(size=16, color="#333333"),
    )
    fig.update_xaxes(showgrid=False, zeroline=False, visible=False)
    fig.update_yaxes(showgrid=False, zeroline=False, visible=False)
    fig.update_layout({"plot_bgcolor": "white", "paper_bgcolor": "white"})
    fig.write_image(f"{dict_name}.svg", format="png")

    fig.show()

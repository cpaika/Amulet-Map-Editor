"""Build a concise PDF report for 6 Trowbridge Circle (noise + air quality)."""
import os, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt, matplotlib.image as mpimg
from matplotlib.backends.backend_pdf import PdfPages

HERE = os.path.dirname(os.path.abspath(__file__))
def img(ax, name):
    ax.imshow(mpimg.imread(os.path.join(HERE, name))); ax.axis("off")

out = os.path.join(HERE, "Trowbridge_noise_report.pdf")
with PdfPages(out) as pdf:
    # ---------- Page 1 : noise ----------
    fig = plt.figure(figsize=(8.5, 11)); fig.patch.set_facecolor("white")
    fig.text(0.5, 0.965, "6 Trowbridge Circle, Shrewsbury MA", ha="center",
             fontsize=18, weight="bold")
    fig.text(0.5, 0.945, "Modelled environmental noise & air quality", ha="center",
             fontsize=11, color="#444")
    ax = fig.add_axes([0.06, 0.46, 0.88, 0.46]); img(ax, "noise_map.png")
    summary = (
        "HOW LOUD IS IT?  Modelled daytime road-traffic noise is about 50–51 dB(A)\n"
        "— a calm suburban level (birdsong with a faint, distant highway hum; well\n"
        "below normal conversation). It is one of the quietest spots in town:\n"
        "quieter than ~79% of Shrewsbury homes (town average ~54 dB). It's a\n"
        "cul-de-sac on a low rise, 1–3 km from every state highway; the loudest\n"
        "nearby source is Main Street (~45 dB, 154 m away). Aircraft is negligible.")
    fig.text(0.06, 0.41, summary, fontsize=10.5, va="top", linespacing=1.55)
    tbl = (
        "Comparison (A-weighted Leq, dB)        vs. Trowbridge\n"
        "  Shrewsbury Town Hall / common   48.5     about the same\n"
        "  6 TROWBRIDGE CIRCLE             50.5     — (reference)\n"
        "  17A EK Court (Half Moon Cove)   50.9     about the same\n"
        "  Sherwood Ave (residential)      54.1     +4  (~1.3x louder)\n"
        "  near I-290 (Reservoir St)       60.2     +10 (~2x louder)\n"
        "  lakeside by Rt 9/290            65.3     +15 (~2.8x louder)\n"
        "  fronting MA-140 (Grafton St)    69.5     +19 (~3.7x louder)\n"
        "  fronting Route 9 (Harrington)   71.4     +21 (~4.2x louder)")
    fig.text(0.06, 0.205, tbl, fontsize=9.2, va="top", family="monospace")
    fig.text(0.06, 0.045,
             "Method: measured MassDOT AADT (FHWA TNM emissions) · 1 m LiDAR terrain · OSM buildings · "
             "Lake Quinsigamond water\nacoustics · aircraft layer · ISO 9613-2 propagation. Cross-validated vs. "
             "USDOT National Transportation Noise Map.\nEvery +10 dB ≈ twice as loud. Estimate, ~±3 dB on absolute level.",
             fontsize=7.6, va="top", color="#555", linespacing=1.4)
    pdf.savefig(fig); plt.close(fig)

    # ---------- Page 2 : house map, fence, air ----------
    fig = plt.figure(figsize=(8.5, 11)); fig.patch.set_facecolor("white")
    fig.text(0.5, 0.965, "6 Trowbridge Circle — detail", ha="center", fontsize=16, weight="bold")
    ax = fig.add_axes([0.07, 0.60, 0.52, 0.33]); img(ax, "trowbridge_noise_preview.png")
    ax2 = fig.add_axes([0.60, 0.62, 0.38, 0.29]); img(ax2, "air_quality_compare.png")
    fig.text(0.07, 0.575, "Noise at ear height on 1 m LiDAR (red = Main St).",
             fontsize=8.5, color="#444")
    fig.text(0.60, 0.605, "Air quality vs. EK Court (regional air identical).",
             fontsize=8.5, color="#444")
    fence = (
        "CAN A NOISE FENCE HELP?  No, not meaningfully.\n"
        "A fence/berm buys ≤1 dB at the house (≤1.6 dB even for a full 3 m perimeter wall) — below the\n"
        "~3 dB needed to notice a change. Reasons (from the LiDAR): (1) the house sits ~5 m ABOVE Main\n"
        "Street on a rise, so an elevated listener sees over any fence; (2) noise arrives from all sides — Main\n"
        "St (N), your own cul-de-sac 23 m to the S, and South St (SW); (3) the loud sources are far and at\n"
        "shallow angles. It's already quiet, so there's little to gain. For a calmer patio, use the house's own\n"
        "shadow (seat away from Main St) + a low wall close to the nearest source; expect ~1–2 dB at best.")
    fig.text(0.07, 0.52, fence, fontsize=9.6, va="top", linespacing=1.5)
    air = (
        "AIR QUALITY.  Generally Good–Moderate (US AQI ~45 avg; ~51% Good days). PM2.5 annual mean\n"
        "~9 µg/m³ — right at the 2024 EPA standard, like most of the Northeast; spikes are summer ozone and\n"
        "wildfire-smoke days. Local traffic adds only ~+2.6 µg/m³ NO₂ (you're far from highways); aircraft from\n"
        "Worcester Airport is negligible. Same theme as noise: distance from major roads keeps both low.")
    fig.text(0.07, 0.235, air, fontsize=9.6, va="top", linespacing=1.5)
    fig.text(0.07, 0.06,
             "Full model, code, data and an interactive 3D map: analysis/shrewsbury_sound/  ·  "
             "estimates for guidance, not a regulatory survey.",
             fontsize=7.8, va="top", color="#555")
    pdf.savefig(fig); plt.close(fig)

print("wrote", out, os.path.getsize(out)//1024, "KB")
